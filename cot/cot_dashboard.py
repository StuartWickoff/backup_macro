"""Build per-market positioning charts from CFTC Legacy Futures reports."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import sys
from pathlib import Path
from typing import Iterable

import matplotlib

matplotlib.use("Agg")

import matplotlib.dates as mdates
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


MARKETS: dict[str, tuple[str, ...]] = {
    "AUD": ("AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE",),
    "NZD": (
        "NEW ZEALAND DOLLAR - CHICAGO MERCANTILE EXCHANGE",
        "NZ DOLLAR - CHICAGO MERCANTILE EXCHANGE",
    ),
    "EUR": ("EURO FX - CHICAGO MERCANTILE EXCHANGE",),
    "GBP": (
        "BRITISH POUND - CHICAGO MERCANTILE EXCHANGE",
        "BRITISH POUND STERLING - CHICAGO MERCANTILE EXCHANGE",
    ),
    "JPY": ("JAPANESE YEN - CHICAGO MERCANTILE EXCHANGE",),
    "CHF": ("SWISS FRANC - CHICAGO MERCANTILE EXCHANGE",),
    "CAD": ("CANADIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE",),
    "USD_INDEX": ("U.S. DOLLAR INDEX - ICE FUTURES U.S.",),
}
USD_DXY_WEIGHTS = {
    "EUR": 0.576,
    "JPY": 0.136,
    "GBP": 0.119,
    "CAD": 0.091,
    "CHF": 0.036,
}
USD_AGGREGATE_COMPONENTS = tuple(USD_DXY_WEIGHTS)

COLUMN_ALIASES: dict[str, tuple[str, ...]] = {
    "market": ("Market and Exchange Names", "Market_and_Exchange_Names"),
    "date": ("As of Date in Form YYYY-MM-DD", "Report_Date_as_MM_DD_YYYY"),
    "long": (
        "Noncommercial Positions-Long (All)",
        "NonComm_Positions_Long_All",
    ),
    "short": (
        "Noncommercial Positions-Short (All)",
        "NonComm_Positions_Short_All",
    ),
    "open_interest": ("Open Interest (All)", "Open_Interest_All"),
}

CHART_TITLES = {
    "AUD": "AUSTRALIAN DOLLAR",
    "NZD": "NZ DOLLAR",
    "EUR": "EURO FX",
    "GBP": "BRITISH POUND",
    "JPY": "JAPANESE YEN",
    "CHF": "SWISS FRANC",
    "CAD": "CANADIAN DOLLAR",
    "USD_INDEX": "U.S. DOLLAR INDEX — proxy DXY pondéré",
    "USD_AGGREGATE": "USD AGGREGATE — proxy COT DXY SEK-free",
}

PURPLE = "#c8a9ff"
FIGURE_BACKGROUND = "#050505"
PLOT_BACKGROUND = "#050505"
ZERO_LINE = "#777777"
GRID = "#262626"
PROJECT_DIR = Path(__file__).resolve().parent
DISPLAY_WINDOW_YEARS = 5


class CotDataError(ValueError):
    """Raised when a CFTC report cannot be interpreted safely."""


def _resolve_columns(columns: Iterable[str]) -> dict[str, str]:
    available = set(columns)
    resolved: dict[str, str] = {}
    missing: list[str] = []
    for field, aliases in COLUMN_ALIASES.items():
        match = next((name for name in aliases if name in available), None)
        if match is None:
            missing.append(f"{field} ({' / '.join(aliases)})")
        else:
            resolved[field] = match
    if missing:
        raise CotDataError("Colonnes CFTC introuvables : " + "; ".join(missing))
    return resolved


def _parse_dates(values: pd.Series) -> pd.Series:
    return pd.to_datetime(values, errors="coerce", format="mixed")


def load_levels(path: Path | None) -> dict[str, tuple[float | None, float | None]]:
    if path is None:
        return {}
    levels = pd.read_csv(path)
    required = {"currency", "lower_level", "upper_level"}
    missing = required.difference(levels.columns)
    if missing:
        raise CotDataError(
            "Colonnes absentes du fichier de niveaux : " + ", ".join(sorted(missing))
        )
    if levels["currency"].duplicated().any():
        raise CotDataError("Le fichier de niveaux contient des devises en double")

    allowed = set(MARKETS)
    unknown = set(levels["currency"].dropna()) - allowed
    if unknown:
        raise CotDataError(
            "Devises inconnues dans le fichier de niveaux : "
            + ", ".join(sorted(unknown))
        )
    result: dict[str, tuple[float | None, float | None]] = {}
    for row in levels.itertuples(index=False):
        try:
            lower = None if pd.isna(row.lower_level) else float(row.lower_level)
            upper = None if pd.isna(row.upper_level) else float(row.upper_level)
        except (TypeError, ValueError) as error:
            raise CotDataError(
                f"Niveaux invalides pour {row.currency}: valeurs numériques attendues"
            ) from error
        if lower is not None and upper is not None and lower >= upper:
            raise CotDataError(
                f"Niveaux inversés pour {row.currency}: lower_level doit être inférieur"
            )
        result[str(row.currency)] = (lower, upper)
    return result


def latest_percentile_levels(
    data: pd.DataFrame, window: int = 52
) -> tuple[float, float] | None:
    """Return user-script P20/P80 levels, distinct from course/CME thresholds."""
    values = data["net_pct_oi"].dropna().astype(float).tail(window)
    if len(values) < window:
        return None
    return float(values.quantile(0.20)), float(values.quantile(0.80))


def export_observations(markets: dict[str, pd.DataFrame], path: Path) -> None:
    exports = []
    for data in markets.values():
        exported = data.copy()
        levels = latest_percentile_levels(exported)
        exported["p20_last_52"] = levels[0] if levels else pd.NA
        exported["p80_last_52"] = levels[1] if levels else pd.NA
        exports.append(exported)
    if not exports:
        raise CotDataError("Aucune observation à exporter")
    path.parent.mkdir(parents=True, exist_ok=True)
    pd.concat(exports, ignore_index=True).to_csv(path, index=False)


def publish_chart_assets(
    markets: dict[str, pd.DataFrame], charts_dir: Path, assets_dir: Path
) -> None:
    assets_dir.mkdir(parents=True, exist_ok=True)
    manifest_markets = []
    for currency, data in markets.items():
        if currency == "USD_INDEX":
            continue
        chart_file = f"{currency.lower()}_net_noncomm_pct_oi.png"
        chart_path = charts_dir / chart_file
        asset_path = assets_dir / chart_file
        if chart_path.resolve() != asset_path.resolve():
            shutil.copyfile(chart_path, asset_path)

        latest_date = pd.Timestamp(data["date"].max())
        manifest_markets.append(
            {
                "currency": (
                    "USD"
                    if currency in {"USD_INDEX", "USD_AGGREGATE"}
                    else currency
                ),
                "market_name": str(data["market_name"].iloc[-1]),
                "chart_file": chart_file,
                "latest_date": latest_date.strftime("%Y-%m-%d"),
                "is_usd_proxy": bool(data["is_usd_proxy"].iloc[-1]),
                "is_usd_aggregate": bool(data["is_usd_aggregate"].iloc[-1]),
                "is_stale": (pd.Timestamp.now().normalize() - latest_date).days > 14,
            }
        )

    manifest_path = assets_dir / "manifest.json"
    temporary_path = manifest_path.with_suffix(".json.tmp")
    try:
        temporary_path.write_text(
            json.dumps({"markets": manifest_markets}, ensure_ascii=False, indent=2)
            + "\n",
            encoding="utf-8",
        )
        os.replace(temporary_path, manifest_path)
    except OSError:
        temporary_path.unlink(missing_ok=True)
        raise


def prepare_market_data(report: pd.DataFrame) -> dict[str, pd.DataFrame]:
    """Return exact-market weekly series; never mix a currency with its crosses."""
    columns = _resolve_columns(report.columns)
    market_names = report[columns["market"]].astype("string").str.strip()
    results: dict[str, pd.DataFrame] = {}

    for currency, aliases in MARKETS.items():
        selected = report.loc[market_names.isin(aliases)].copy()
        if selected.empty:
            continue

        selected["date"] = _parse_dates(selected[columns["date"]])
        selected["market_name"] = (
            selected[columns["market"]].astype("string").str.strip()
        )
        selected["long_noncomm"] = pd.to_numeric(
            selected[columns["long"]], errors="coerce"
        )
        selected["short_noncomm"] = pd.to_numeric(
            selected[columns["short"]], errors="coerce"
        )
        selected["open_interest"] = pd.to_numeric(
            selected[columns["open_interest"]], errors="coerce"
        )
        selected = selected.dropna(
            subset=["date", "long_noncomm", "short_noncomm", "open_interest"]
        )
        selected = selected.sort_values("date")

        duplicate_dates = selected["date"].duplicated(keep=False)
        if duplicate_dates.any():
            dates = selected.loc[duplicate_dates, "date"].dt.strftime("%Y-%m-%d")
            raise CotDataError(
                f"{currency}: plusieurs lignes CFTC pour une même date : "
                + ", ".join(dates.unique())
            )

        selected["net_noncomm"] = (
            selected["long_noncomm"] - selected["short_noncomm"]
        )
        valid_open_interest = selected["open_interest"] > 0
        selected["net_pct_oi"] = pd.NA
        selected.loc[valid_open_interest, "net_pct_oi"] = (
            selected.loc[valid_open_interest, "net_noncomm"]
            / selected.loc[valid_open_interest, "open_interest"]
            * 100
        )
        selected["currency"] = currency
        selected["is_usd_proxy"] = currency == "USD_INDEX"
        selected["is_usd_aggregate"] = False
        results[currency] = selected[
            [
                "date",
                "market_name",
                "currency",
                "long_noncomm",
                "short_noncomm",
                "open_interest",
                "net_noncomm",
                "net_pct_oi",
                "is_usd_proxy",
                "is_usd_aggregate",
            ]
        ].reset_index(drop=True)

    if not results:
        available_markets = sorted(market_names.dropna().unique())
        expected = sorted(alias for aliases in MARKETS.values() for alias in aliases)
        raise CotDataError(
            "Aucun marché suivi trouvé par correspondance exacte. "
            f"Marchés attendus : {expected}. "
            f"Exemples dans le fichier : {available_markets[:12]}"
        )
    return results


def build_usd_aggregate(markets: dict[str, pd.DataFrame]) -> pd.DataFrame:
    """Build a SEK-free USD COT proxy using the covered DXY currency weights."""
    missing = set(USD_AGGREGATE_COMPONENTS).difference(markets)
    if missing:
        raise CotDataError(
            "USD Aggregate impossible : marchés manquants "
            + ", ".join(sorted(missing))
        )

    components: pd.DataFrame | None = None
    for currency in USD_AGGREGATE_COMPONENTS:
        observations = markets[currency][["date", "net_pct_oi"]].copy()
        observations["net_pct_oi"] = pd.to_numeric(
            observations["net_pct_oi"], errors="coerce"
        )
        observations = observations.rename(
            columns={"net_pct_oi": f"{currency.lower()}_net_pct_oi"}
        )
        components = (
            observations
            if components is None
            else components.merge(observations, on="date", how="inner", validate="one_to_one")
        )

    if components is None:
        raise CotDataError("USD Aggregate impossible : aucune devise constituante")

    component_columns = [
        f"{currency.lower()}_net_pct_oi" for currency in USD_AGGREGATE_COMPONENTS
    ]
    components = components.dropna(subset=component_columns)
    if components.empty:
        raise CotDataError(
            "USD Aggregate impossible : aucune date commune avec les 5 marchés DXY suivis"
        )

    weighted_net_pct_oi = sum(
        components[f"{currency.lower()}_net_pct_oi"] * weight
        for currency, weight in USD_DXY_WEIGHTS.items()
    )
    aggregate = pd.DataFrame(
        {
            "date": components["date"],
            "market_name": (
                "USD COT PROXY — inverse DXY weights, SEK-free "
                "(95.8% covered; no renormalization)"
            ),
            "currency": "USD_AGGREGATE",
            "long_noncomm": pd.NA,
            "short_noncomm": pd.NA,
            "open_interest": pd.NA,
            "net_noncomm": pd.NA,
            "net_pct_oi": -weighted_net_pct_oi,
            "is_usd_proxy": False,
            "is_usd_aggregate": True,
        }
    )
    return aggregate.reset_index(drop=True)


def plot_market(
    currency: str,
    data: pd.DataFrame,
    output_path: Path,
    *,
    lower_level: float | None = None,
    upper_level: float | None = None,
    smooth: bool = False,
) -> None:
    """Render one historical curve; supplied colored levels are never inferred."""
    valid = data.dropna(subset=["date", "net_pct_oi"]).sort_values("date")
    if valid.empty:
        raise CotDataError(f"{currency}: aucune observation valide à tracer")

    dates = pd.to_datetime(valid["date"])
    values = valid["net_pct_oi"].astype(float)
    fig, axis = plt.subplots(figsize=(18, 9), dpi=200)
    fig.patch.set_facecolor(FIGURE_BACKGROUND)
    axis.set_facecolor(PLOT_BACKGROUND)

    if smooth and len(valid) >= 4:
        x = mdates.date2num(dates)
        dense_x = np.linspace(x[0], x[-1], (len(x) - 1) * 8 + 1)
        dense_values = _catmull_rom(values.to_numpy(), len(dense_x))
        line_dates = mdates.num2date(dense_x)
        line_values = dense_values
    else:
        line_dates = dates
        line_values = values

    axis.plot(line_dates, line_values, color=PURPLE, linewidth=1.5, label="Net % OI")
    axis.fill_between(
        line_dates,
        line_values,
        0,
        where=np.asarray(line_values) <= 0,
        color=PURPLE,
        alpha=0.08,
        interpolate=True,
    )
    axis.fill_between(
        line_dates,
        line_values,
        0,
        where=np.asarray(line_values) > 0,
        color="#9b5365",
        alpha=0.12,
        interpolate=True,
    )
    axis.axhline(0, color=ZERO_LINE, linewidth=0.9, linestyle=(0, (6, 6)))
    if dates.iloc[0] < dates.iloc[-1]:
        window_start = dates.iloc[-1] - pd.DateOffset(years=DISPLAY_WINDOW_YEARS)
        visible_start = dates.iloc[dates.searchsorted(window_start)]
        axis.set_xlim(visible_start, dates.iloc[-1])

    if lower_level is not None:
        axis.axhline(
            lower_level,
            color="#00c853",
            linewidth=0.9,
            linestyle=(0, (2, 4)),
        )
    if upper_level is not None:
        axis.axhline(
            upper_level,
            color="#ff3d3d",
            linewidth=0.9,
            linestyle=(0, (2, 4)),
        )

    axis.set_title(
        f"{CHART_TITLES.get(currency, currency)} — Net Non-Comm % OI",
        color="#eeeeee",
        fontsize=11,
        pad=18,
    )
    axis.set_ylabel("% of Open Interest", color="#bbbbbb", fontsize=9)
    axis.xaxis.set_major_locator(mdates.MonthLocator(bymonth=[1, 7]))
    axis.xaxis.set_major_formatter(mdates.DateFormatter("%b %Y"))
    axis.tick_params(colors="#b0b0b0", labelsize=8, length=0)
    axis.grid(axis="both", color=GRID, linewidth=0.55, alpha=0.55)
    for spine in axis.spines.values():
        spine.set_visible(False)
    axis.legend(
        loc="upper left",
        bbox_to_anchor=(0, 1.13),
        frameon=False,
        labelcolor="#cccccc",
        fontsize=7,
    )
    fig.subplots_adjust(left=0.045, right=0.955, top=0.88, bottom=0.09)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(output_path, facecolor=FIGURE_BACKGROUND)
    plt.close(fig)


def _catmull_rom(values: np.ndarray, output_count: int) -> np.ndarray:
    """Smooth only the rendered path; original weekly values remain untouched."""
    points = np.asarray(values, dtype=float)
    target_x = np.linspace(0.0, 1.0, output_count)
    result = np.empty(output_count)
    for index, position in enumerate(target_x):
        segment = min(int(position * (len(points) - 1)), len(points) - 2)
        fraction = position * (len(points) - 1) - segment
        previous = points[max(segment - 1, 0)]
        current = points[segment]
        following = points[segment + 1]
        after = points[min(segment + 2, len(points) - 1)]
        result[index] = 0.5 * (
            2 * current
            + (-previous + following) * fraction
            + (2 * previous - 5 * current + 4 * following - after) * fraction**2
            + (-previous + 3 * current - 3 * following + after) * fraction**3
        )
    return result


def download_reports(
    start_year: int,
    end_year: int,
    cache_dir: Path,
    *,
    refresh: bool = False,
) -> pd.DataFrame:
    reports = []
    cot_reports = None
    for year in range(start_year, end_year + 1):
        cache_file = cache_dir / f"legacy_futures_{year}.csv"
        if cache_file.is_file() and not (refresh or year == end_year):
            reports.append(pd.read_csv(cache_file, low_memory=False))
            print(f"CFTC {year}: cache", file=sys.stderr)
            continue

        if cot_reports is None:
            try:
                import cot_reports as cot_reports_package
            except ImportError as error:
                raise CotDataError(
                    "Le paquet cot-reports manque. "
                    "Installez les dépendances de cot/requirements.txt."
                ) from error
            cot_reports = cot_reports_package

        print(f"Téléchargement du rapport CFTC {year}…", file=sys.stderr)
        try:
            report = cot_reports.cot_year(
                year=year,
                cot_report_type="legacy_fut",
                store_txt=False,
                verbose=False,
            )
        except Exception as error:
            if cache_file.is_file():
                print(
                    f"Avertissement : téléchargement CFTC {year} échoué "
                    f"({error}); utilisation du cache {cache_file}.",
                    file=sys.stderr,
                )
                reports.append(pd.read_csv(cache_file, low_memory=False))
                continue
            raise CotDataError(
                f"Échec du téléchargement CFTC {year}: {error}"
            ) from error
        cache_dir.mkdir(parents=True, exist_ok=True)
        temporary_file = cache_file.with_suffix(".csv.tmp")
        try:
            report.to_csv(temporary_file, index=False)
            os.replace(temporary_file, cache_file)
        except OSError as error:
            temporary_file.unlink(missing_ok=True)
            raise CotDataError(
                f"Impossible d'écrire le cache CFTC {year}: {error}"
            ) from error
        reports.append(report)
    if not reports:
        raise CotDataError("Aucun rapport CFTC n'a été téléchargé.")
    return pd.concat(reports, ignore_index=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--csv", type=Path, help="Fichier CFTC CSV déjà téléchargé")
    parser.add_argument("--start-year", type=int, default=2021)
    parser.add_argument("--end-year", type=int, default=pd.Timestamp.now().year)
    parser.add_argument(
        "--output-dir", type=Path, default=PROJECT_DIR / "output"
    )
    parser.add_argument(
        "--web-assets-dir",
        type=Path,
        help="Copier les graphiques et le manifeste dans les ressources statiques du dashboard",
    )
    parser.add_argument(
        "--cache-dir",
        type=Path,
        default=PROJECT_DIR / ".cache" / "cftc",
        help="Cache local des rapports CFTC annuels",
    )
    parser.add_argument(
        "--refresh-cache",
        action="store_true",
        help="Retélécharger aussi les années déjà en cache",
    )
    parser.add_argument("--as-of", type=pd.Timestamp, help="Dernière date de référence incluse")
    parser.add_argument(
        "--no-percentile-bands",
        action="store_true",
        help="Masquer les P20/P80 des 52 dernières observations",
    )
    parser.add_argument(
        "--levels-csv",
        type=Path,
        help="Seuils externes connus, colonnes currency/lower_level/upper_level",
    )
    parser.add_argument(
        "--smooth",
        action="store_true",
        help="Lisser uniquement le tracé visuel, sans modifier les données",
    )
    args = parser.parse_args()

    try:
        if args.csv:
            report = pd.read_csv(args.csv, low_memory=False)
        else:
            if args.start_year > args.end_year:
                raise CotDataError("--start-year doit être inférieur ou égal à --end-year")
            report = download_reports(
                args.start_year,
                args.end_year,
                args.cache_dir,
                refresh=args.refresh_cache,
            )
        markets = prepare_market_data(report)
        if args.as_of is not None:
            markets = {
                currency: data.loc[data["date"] <= args.as_of].reset_index(drop=True)
                for currency, data in markets.items()
            }
        markets["USD_AGGREGATE"] = build_usd_aggregate(markets)
        levels = load_levels(args.levels_csv)
        csv_output = args.output_dir / "cot_observations.csv"
        export_observations(markets, csv_output)
        for currency, data in markets.items():
            destination = args.output_dir / f"{currency.lower()}_net_noncomm_pct_oi.png"
            lower, upper = levels.get(currency, (None, None))
            if not args.no_percentile_bands and currency not in levels:
                percentiles = latest_percentile_levels(data)
                if percentiles is not None:
                    lower, upper = percentiles
            plot_market(
                currency,
                data,
                destination,
                lower_level=lower,
                upper_level=upper,
                smooth=args.smooth,
            )
            print(f"{currency}: {len(data)} observations → {destination}")
        if args.web_assets_dir:
            publish_chart_assets(markets, args.output_dir, args.web_assets_dir)
            print(f"Graphiques du dashboard publiés → {args.web_assets_dir}")
        print(f"Données exportées → {csv_output}")
        return 0
    except (CotDataError, OSError, pd.errors.ParserError) as error:
        print(f"Erreur COT : {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
