import io
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np
import pandas as pd
from PIL import Image

from cot_dashboard import (
    CotDataError,
    USD_AGGREGATE_COMPONENTS,
    USD_DXY_WEIGHTS,
    build_usd_aggregate,
    export_observations,
    latest_percentile_levels,
    load_levels,
    plot_market,
    publish_chart_assets,
    prepare_market_data,
    download_reports,
)


def report_frame(*, underscored: bool = False) -> pd.DataFrame:
    if underscored:
        return pd.DataFrame(
            {
                "Market_and_Exchange_Names": [
                    "AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE",
                    "EURO FX/AUSTRALIAN DOLLAR XRATE - CHICAGO MERCANTILE EXCHANGE",
                    "AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE",
                ],
                "Report_Date_as_MM_DD_YYYY": [
                    "01/06/2026",
                    "01/06/2026",
                    "01/13/2026",
                ],
                "NonComm_Positions_Long_All": [120, 999, 110],
                "NonComm_Positions_Short_All": [40, 0, 50],
                "Open_Interest_All": [400, 1, 0],
            }
        )
    return pd.DataFrame(
        {
            "Market and Exchange Names": [
                "AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE",
                "EURO FX/AUSTRALIAN DOLLAR XRATE - CHICAGO MERCANTILE EXCHANGE",
            ],
            "As of Date in Form YYYY-MM-DD": ["2026-01-06", "2026-01-06"],
            "Noncommercial Positions-Long (All)": [120, 999],
            "Noncommercial Positions-Short (All)": [40, 0],
            "Open Interest (All)": [400, 1],
        }
    )


class PrepareMarketDataTests(unittest.TestCase):
    def test_exact_market_selection_and_net_percent_open_interest(self) -> None:
        markets = prepare_market_data(report_frame())

        self.assertEqual(list(markets), ["AUD"])
        self.assertEqual(float(markets["AUD"].iloc[0]["net_pct_oi"]), 20.0)
        self.assertEqual(float(markets["AUD"].iloc[0]["net_noncomm"]), 80.0)

    def test_supports_alternate_cot_reports_column_names(self) -> None:
        market = prepare_market_data(report_frame(underscored=True))["AUD"]

        self.assertEqual(len(market), 2)
        self.assertEqual(float(market.iloc[0]["net_pct_oi"]), 20.0)
        self.assertTrue(pd.isna(market.iloc[1]["net_pct_oi"]))

    def test_duplicate_market_dates_are_rejected(self) -> None:
        report = pd.concat([report_frame(), report_frame().iloc[[0]]])

        with self.assertRaisesRegex(CotDataError, "plusieurs lignes"):
            prepare_market_data(report)

    def test_missing_columns_are_reported(self) -> None:
        with self.assertRaisesRegex(CotDataError, "Colonnes CFTC introuvables"):
            prepare_market_data(pd.DataFrame({"unrelated": [1]}))

    def test_latest_percentile_levels_need_a_full_window(self) -> None:
        market = prepare_market_data(report_frame())["AUD"]

        self.assertIsNone(latest_percentile_levels(market, window=52))

    def test_latest_percentiles_use_the_last_52_values(self) -> None:
        market = pd.DataFrame({"net_pct_oi": range(52)})

        low, high = latest_percentile_levels(market)
        self.assertAlmostEqual(low, 10.2)
        self.assertAlmostEqual(high, 40.8)

    def test_export_contains_observations_and_current_bands(self) -> None:
        markets = {"AUD": pd.DataFrame({"net_pct_oi": range(52)})}
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "cot_observations.csv"

            export_observations(markets, output)

            saved = pd.read_csv(output)
            self.assertAlmostEqual(float(saved.iloc[0]["p20_last_52"]), 10.2)
            self.assertAlmostEqual(float(saved.iloc[-1]["p80_last_52"]), 40.8)

    def test_published_assets_include_market_dates_and_usd_proxy_metadata(self) -> None:
        markets = {
            "AUD": pd.DataFrame(
                {
                    "date": [pd.Timestamp("2026-10-27")],
                    "market_name": [
                        "AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE"
                    ],
                    "is_usd_proxy": [False],
                    "is_usd_aggregate": [False],
                }
            ),
            "USD_INDEX": pd.DataFrame(
                {
                    "date": [pd.Timestamp("2022-02-01")],
                    "market_name": ["U.S. DOLLAR INDEX - ICE FUTURES U.S."],
                    "is_usd_proxy": [True],
                    "is_usd_aggregate": [False],
                }
            ),
            "USD_AGGREGATE": pd.DataFrame(
                {
                    "date": [pd.Timestamp("2026-10-27")],
                    "market_name": ["Synthetic USD Aggregate"],
                    "is_usd_proxy": [False],
                    "is_usd_aggregate": [True],
                }
            ),
        }
        with tempfile.TemporaryDirectory() as directory:
            charts = Path(directory) / "charts"
            assets = Path(directory) / "web" / "cot"
            charts.mkdir()
            for currency in markets:
                (charts / f"{currency.lower()}_net_noncomm_pct_oi.png").write_bytes(
                    b"chart"
                )

            publish_chart_assets(
                markets, charts, assets, as_of=pd.Timestamp("2026-10-31")
            )

            manifest = json.loads((assets / "manifest.json").read_text(encoding="utf-8"))
            by_currency = {market["currency"]: market for market in manifest["markets"]}
            self.assertEqual(by_currency["AUD"]["latest_date"], "2026-10-27")
            self.assertFalse(by_currency["AUD"]["is_stale"])
            self.assertEqual(by_currency["USD"]["latest_date"], "2026-10-27")
            self.assertTrue(by_currency["USD"]["is_usd_aggregate"])
            self.assertFalse(by_currency["USD"]["is_usd_proxy"])
            self.assertFalse(by_currency["USD"]["is_stale"])
            self.assertTrue(
                (assets / "usd_aggregate_net_noncomm_pct_oi.png").is_file()
            )
            self.assertFalse((assets / "usd_index_net_noncomm_pct_oi.png").exists())

            # La reference est injectable : loin de la dernière observation,
            # la staleness passe à TRUE quels que soient le calendrier reel.
            publish_chart_assets(
                markets, charts, assets, as_of=pd.Timestamp("2027-01-31")
            )
            manifest_later = json.loads(
                (assets / "manifest.json").read_text(encoding="utf-8")
            )
            by_currency_later = {
                market["currency"]: market for market in manifest_later["markets"]
            }
            self.assertTrue(by_currency_later["AUD"]["is_stale"])
            self.assertTrue(by_currency_later["USD"]["is_stale"])

    def test_usd_aggregate_is_the_inverse_of_covered_dxy_weighted_currencies(self) -> None:
        markets = {
            currency: pd.DataFrame(
                {
                    "date": pd.to_datetime(["2026-10-20", "2026-10-27"]),
                    "net_pct_oi": [index * 10, index * 10 + 5],
                }
            )
            for index, currency in enumerate(USD_AGGREGATE_COMPONENTS, start=1)
        }

        aggregate = build_usd_aggregate(markets)
        expected_first = -sum(
            index * 10 * weight
            for index, weight in enumerate(USD_DXY_WEIGHTS.values(), start=1)
        )
        expected_second = -sum(
            (index * 10 + 5) * weight
            for index, weight in enumerate(USD_DXY_WEIGHTS.values(), start=1)
        )

        self.assertEqual(aggregate["currency"].unique().tolist(), ["USD_AGGREGATE"])
        self.assertAlmostEqual(float(aggregate.iloc[0]["net_pct_oi"]), expected_first)
        self.assertAlmostEqual(float(aggregate.iloc[1]["net_pct_oi"]), expected_second)
        self.assertAlmostEqual(sum(USD_DXY_WEIGHTS.values()), 0.958)
        self.assertIn("SEK-free", aggregate.iloc[0]["market_name"])
        self.assertTrue(aggregate["is_usd_aggregate"].all())
        self.assertTrue(aggregate["long_noncomm"].isna().all())

    def test_usd_aggregate_requires_all_five_dxy_currency_markets(self) -> None:
        with self.assertRaisesRegex(CotDataError, "marchés manquants"):
            build_usd_aggregate({"AUD": pd.DataFrame()})

    def test_threshold_file_accepts_values_but_not_invented_ones(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "levels.csv"
            path.write_text(
                "currency,lower_level,upper_level\nAUD,-25,20\n",
                encoding="utf-8",
            )

            self.assertEqual(load_levels(path), {"AUD": (-25.0, 20.0)})

            path.write_text(
                "currency,lower_level,upper_level\nAUD,20,-25\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(CotDataError, "Niveaux inversés"):
                load_levels(path)

    def test_annual_reports_are_cached_and_current_year_is_refreshed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            cache = Path(directory)
            cached = report_frame()
            cached.to_csv(cache / "legacy_futures_2025.csv", index=False)
            downloaded = report_frame().assign(
                **{"As of Date in Form YYYY-MM-DD": ["2026-01-13", "2026-01-13"]}
            )
            with patch("cot_reports.cot_year", return_value=downloaded) as cot_year:
                report = download_reports(2025, 2026, cache)

            self.assertEqual(len(report), len(cached) + len(downloaded))
            cot_year.assert_called_once_with(
                year=2026,
                cot_report_type="legacy_fut",
                store_txt=False,
                verbose=False,
            )
            self.assertTrue((cache / "legacy_futures_2026.csv").is_file())

    def test_current_year_download_failure_uses_and_warns_about_cached_report(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            cache = Path(directory)
            cached = report_frame()
            cached.to_csv(cache / "legacy_futures_2026.csv", index=False)
            with (
                patch("cot_reports.cot_year", side_effect=RuntimeError("offline")),
                patch("sys.stderr", new_callable=io.StringIO) as stderr,
            ):
                report = download_reports(2026, 2026, cache)

            self.assertEqual(len(report), len(cached))
            self.assertTrue(
                (report["As of Date in Form YYYY-MM-DD"] == "2026-01-06").all()
            )
            self.assertIn("utilisation du cache", stderr.getvalue())


class PlotMarketTests(unittest.TestCase):
    def test_chart_shows_five_year_window(self) -> None:
        market = pd.DataFrame(
            {
                "date": pd.date_range("2021-01-05", periods=296, freq="W-TUE"),
                "net_pct_oi": np.sin(np.linspace(0, 16, 296)) * 40,
            }
        )
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "five_year_window.png"

            plot_market("NZD", market, output)

            with Image.open(output) as image:
                pixels = np.asarray(image.convert("RGB"))
            curve_pixels = np.all(pixels == (200, 169, 255), axis=2)
            curve_x = np.where(curve_pixels)[1]
            self.assertLess(int(curve_x.min()), 220)
            self.assertGreater(int(curve_x.max()), 3400)

    def test_renders_reference_aspect_ratio_and_optional_levels(self) -> None:
        market = prepare_market_data(report_frame())["AUD"]
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "aud.png"

            plot_market(
                "AUD",
                market,
                output,
                lower_level=-20,
                upper_level=20,
                smooth=True,
            )

            with Image.open(output) as image:
                self.assertEqual(image.size, (3600, 1800))
                pixels = np.asarray(image.convert("RGB"))
                self.assertTrue(np.any(np.all(pixels == (0, 200, 83), axis=2)))
                self.assertTrue(np.any(np.all(pixels == (255, 61, 61), axis=2)))

    def test_levels_are_optional(self) -> None:
        market = prepare_market_data(report_frame())["AUD"]
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "aud.png"

            plot_market("AUD", market, output)

            self.assertTrue(output.is_file())


if __name__ == "__main__":
    unittest.main()
