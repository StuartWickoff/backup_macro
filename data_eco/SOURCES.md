# SOURCES.md — Journal des sources de `data_eco/`

Collecte réalisée le **4 novembre 2026**. Chaque valeur provient exclusivement des
sources listées ci-dessous ; aucune valeur n'a été inventée ou interpolée.
Priorité demandée : 1) Investing.com (valeurs « Actual ») — **inaccessible depuis cet
environnement (HTTP 403, protection anti-robot ; aucun contournement tenté)** ; 2) sources
officielles (banques centrales, instituts statistiques, S&P Global). Les agrégateurs
FRED/OECD et DBnomics ont servi d'accès techniques vers des séries officielles lorsque le
site primaire bloquait l'accès programmatique — chaque usage est signalé.

## Conventions

- Format : `series_id,date,value,value_secondary` ; dates ISO (`YYYY-MM-DD`).
- Séries mensuelles : date = **1er jour du mois d'observation**.
- Séries trimestrielles : date = 1er jour du **mois de fin de trimestre** (T1→`03-01`,
  T2→`06-01`, T3→`09-01`, T4→`12-01`).
- `policy_rate` : taux **en vigueur au 1er jour du mois** (dernier changement ≤ au 1er).
- `real_gdp_growth` : `value` = croissance t/t-1 (CVS), `value_secondary` = croissance
  an/an (publiée ou calculée par capitalisation/niveaux, signalé par série).
- Calculs signalés « YoY calculé » : (I_t/I_t-12 − 1)×100 sur indices officiels, ou
  capitalisation des 4 taux trimestriels, arrondi à 1 décimale.
- Fenêtre cible : 2023-11 → 2026-10 (mensuel) et 2023T4 → 2026T3 (trimestriel) ;
  la couverture réelle dépend des calendriers de publication (cf. CONTROL_SUMMARY.md).


## EUR

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | BCE — Deposit facility rate, date of changes (niveau, %) — Data Portal API — Taux en vigueur au 1er jour du mois (dernier changement ≤ au 1er). | https://data-api.ecb.europa.eu/service/data/FM/B.U2.EUR.4F.KR.DFR.LEV?format=csvdata&startPeriod=2023-08-01 | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Eurostat ei_cphi_m — Total, variation annuelle %, EA20 — Série euro-indicateurs ; % t/t-12. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=EA20&indic=TOTAL&unit=RT12&sinceTimePeriod=2023-11 | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Eurostat ei_cphi_m — HICP - All items excluding energy, food, alcohol and tobacco, variation annuelle %, EA20 — Série euro-indicateurs ; % t/t-12. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=EA20&indic=CP-HI00XEF&unit=RT12&sinceTimePeriod=2023-11 | 2026-11-04 | 〃 |
| `ppi_yoy` | Eurostat sts_inpp_m, EA20, B-D (industrie hors construction), PCH_SM — Unité: ['Percentage change compared to same period in previous year'] | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/sts_inpp_m?format=JSON&lang=EN&geo=EA20&nace_r2=B-D&s_adj=NSA&unit=PCH_SM&sinceTimePeriod=2023-11 | 2026-11-04 | 34 obs, 2023-11-01 → 2026-08-01 |
| `ppi_yoy` | Eurostat sts_inpp_m — prix à la production industrielle (B-D), % variation annuelle, EA21 — Industrie hors construction, données CVS/brutes NSA. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/sts_inpp_m?format=JSON&lang=EN&geo=EA21&nace_r2=B-D&s_adj=NSA&unit=PCH_SM&sinceTimePeriod=2023-11 | 2026-11-04 | 〃 |
| `unemployment_rate` | Eurostat une_rt_m — taux de chômage, % population active, EA21, corrigé des variations saisonnières — Géo EA21 (inclut la Bulgarie depuis 2026-01). | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/une_rt_m?format=JSON&lang=EN&geo=EA21&age=TOTAL&sex=T&unit=PC_ACT&s_adj=SA&sinceTimePeriod=2023-11 | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `real_gdp_growth` | Eurostat namq_10_gdp — PIB réel B1GQ, unit=CLV_PCH_PRE, EA20 — Volume chaîné. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/namq_10_gdp?format=JSON&lang=EN&geo=EA20&na_item=B1GQ&unit=CLV_PCH_PRE&sinceTimePeriod=2023-Q4 | 2026-11-04 | 12 obs, 2023-12-01 → 2026-09-01 |
| `real_gdp_growth` | Eurostat namq_10_gdp — PIB réel B1GQ, unit=CLV_PCH_SM, EA20 — Volume chaîné, même trimestre année précédente. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/namq_10_gdp?format=JSON&lang=EN&geo=EA20&na_item=B1GQ&unit=CLV_PCH_SM&sinceTimePeriod=2023-Q4 | 2026-11-04 | 〃 |
| `sp_global_manufacturing_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Eurozone Manufacturing », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 1 obs, 2026-10-01 → 2026-10-01 |

## GBP

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Bank of England — Official Bank Rate history (table Bank-Rate.asp), % — Taux en vigueur au 1er jour du mois ; changements saisis depuis la table officielle (03/08/23 5,25 ; 01/08/24 5,00 ; 07/11/24 4,75 ; 06/02/25 4,50 ; 08/05/25 4,25 ; 07/08/25 4,00 ; 18/12/25 3,75 — aucun changement en 2026 à la date de collecte). | https://www.bankofengland.co.uk/boeapps/database/Bank-Rate.asp | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | ONS MM23 — CPI annual rate, all items (D7G7), % — Variation annuelle. | https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/d7g7/data?format=csv | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `inflation_rate` | ONS MM23 — CPI 12mth excluding energy, food, alcoholic beverages & tobacco (mm23.csv) — Variation annuelle ; fichier bulk mm23.csv. | https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceindices | 2026-11-04 | 〃 |
| `ppi_yoy` | ONS — PPI output prices, all manufactured products (GB7S), indice 2015=100 — YoY calculé : (I_t/I_t-12 − 1)×100, arrondi à 1 décimale. | https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/gb7s/data?format=csv | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `unemployment_rate` | ONS MGSX — Unemployment rate (aged 16 and over, seasonally adjusted), % — Enquête sur la population active, série mensuelle. | https://www.ons.gov.uk/employmentandlabourmarket/peoplenotinwork/unemployment/timeseries/mgsx/data?format=csv | 2026-11-04 | 33 obs, 2023-11-01 → 2026-07-01 |
| `real_gdp_growth` | ONS IHYQ — GDP quarter on quarter growth, CVM SA % | https://www.ons.gov.uk/economy/grossdomesticproductgdp/timeseries/ihyq/data?format=csv | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |
| `real_gdp_growth` | ONS IHYP — GDP year on year growth, CVM SA % | https://www.ons.gov.uk/economy/grossdomesticproductgdp/timeseries/ihyp/data?format=csv | 2026-11-04 | 〃 |
| `real_gdp_growth` | ONS IHYQ — YoY calculé par capitalisation des 4 taux t/t-1 — Calcul : Π(1+g_t/100) − 1 sur les 4 trimestres, arrondi à 1 décimale. | https://www.ons.gov.uk/economy/grossdomesticproductgdp/timeseries/ihyq/data?format=csv | 2026-11-04 | 〃 |
| `sp_global_manufacturing_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Uk Manufacturing », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-09-01 → 2026-10-01 |
| `sp_global_services_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Uk Services », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-09-01 → 2026-10-01 |

## JPY

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Bank of Japan — taux directeur (taux call overnight non garanti / facilité de dépôt complémentaire), communiqués des MPM — Taux en vigueur au 1er du mois. -0,1 % jusqu'au 19/03/2024 ; 0,1 % (nouveau cadre, cible 0 à 0,1 %) ; 0,25 % (31/07/24) ; 0,5 % (24/01/25) ; 0,75 % (19/12/25) ; 1,0 % (16/06/26) ; 1,25 % (18/09/26). | https://www.boj.or.jp/en/mopo/mpmdeci/index.htm | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Statistics Bureau of Japan — CPI All items (STATJP/CPIm via DBnomics), YoY calculé — Indice (base 2020=100) ; YoY = (I_t/I_t-12 − 1)×100, arrondi à 1 décimale. | https://api.db.nomics.world/v22/series/STATJP/CPIm/001?observations=1&format=json | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `inflation_rate` | Statistics Bureau of Japan — CPI All items, less fresh food (STATJP/CPIm via DBnomics), YoY calculé — « All items, less fresh food » = CPI sous-jacent officiel du Japon. | https://api.db.nomics.world/v22/series/STATJP/CPIm/733?observations=1&format=json | 2026-11-04 | 〃 |
| `ppi_yoy` | Bank of Japan — Corporate Goods Price Index, All commodities, % variation annuelle — Extraits des PDF mensuels cgpiYYMM.pdf (colonne « Yearly » = YoY). | https://www.boj.or.jp/en/statistics/pi/cgpi_release/index.htm | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `unemployment_rate` | Statistics Bureau of Japan — Monthly – Unemployment rate – Both sexes – Percent – seasonally adjusted (STATJP/MIm via DBnomics) — Enquête sur la force de travail. | https://api.db.nomics.world/v22/series/STATJP/MIm/M.UR.B.PCT.SA?observations=1&format=json | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `real_gdp_growth` | Cabinet Office ESRI — Quarterly Estimates of GDP, réel, dépenses ; ritu-jk (t/t-1 CVS) et ritu-jg (YoY), release 2026Q2 2e estim. (qe262_2) — Dernier point : 2026T2 (le T3 paraît mi-novembre). | https://www.esri.cao.go.jp/en/sna/data/sokuhou/files/toukei_top.html | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |
| `sp_global_manufacturing_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Japan Manufacturing », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-09-01 → 2026-10-01 |
| `sp_global_services_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Japan Services », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-08-01 → 2026-09-01 |

## CHF

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Banque nationale suisse — taux directeur (portail de données, cube snbgwdzid, dimension « SNB policy rate ») — Taux en vigueur au 1er jour du mois (dernier changement ≤ au 1er). | https://data.snb.ch/api/cube/snbgwdzid/data/csv/en | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Eurostat ei_cphi_m geo=CH — HICP Total, variation annuelle % (IPC harmonisé suisse, compilé par l'OFS) — Choix de l'HICP : le sous-jacent de l'IPC national (LIK) n'est pas accessible programmatiquement ; l'HICP headline national est disponible via le portail BNS (cube plkopr) et sert de contrôle. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=CH&indic=TOTAL&unit=RT12&sinceTimePeriod=2023-11 | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `inflation_rate` | Eurostat ei_cphi_m geo=CH — HICP HICP - All items excluding energy, food, alcohol and tobacco, variation annuelle % (IPC harmonisé suisse, compilé par l'OFS) — Choix de l'HICP : le sous-jacent de l'IPC national (LIK) n'est pas accessible programmatiquement ; l'HICP headline national est disponible via le portail BNS (cube plkopr) et sert de contrôle. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=CH&indic=CP-HI00XEF&unit=RT12&sinceTimePeriod=2023-11 | 2026-11-04 | 〃 |
| `ppi_yoy` | Eurostat sts_inpp_m geo=CH — indice des prix à la production (B-D), % variation annuelle (source primaire : OFS) — Industrie hors construction. | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/sts_inpp_m?format=JSON&lang=EN&geo=CH&nace_r2=B-D&s_adj=NSA&unit=PCH_SM&sinceTimePeriod=2023-11 | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `unemployment_rate` | FRED (OECD Infra-Annual Labor Statistics) LRHUTTTTCHQ156S — taux de chômage trimestriel CVS, % (source primaire : SECO / définition BIT) — Série trimestrielle (l'enquête suisse est mensuelle mais l'accès direct SECO n'est pas disponible ici). | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTCHQ156S | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |
| `real_gdp_growth` | FRED (OECD National Accounts) NAEXKP01CHQ657S — PIB réel, croissance t/t-1 CVS % (source primaire : SECO, comptes nationaux trimestriels) — YoY calculé par capitalisation des 4 taux trimestriels. Contrôle croisé effectué avec SECO/DBnomics (qna_e_csa). | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CHQ657S | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |

## CAD

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Banque du Canada — taux cible du financement à un jour (série CBC20210 « Target for the overnight rate », API Valet) — Taux en vigueur au 1er jour du mois (dernier changement ≤ au 1er). | https://www.bankofcanada.ca/valet/observations/CBC20210/json?start_date=2023-10-01 | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Banque du Canada — CPI-trim et CPI-median, % variation annuelle (API Valet) — Mesures d'inflation fondamentale ; value=trim, value_secondary=median. | https://www.bankofcanada.ca/valet/observations/CPI_TRIM/json?start_date=2023-10-01 | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `unemployment_rate` | FRED (OECD Infra-Annual Labor Statistics) LRHUTTTTCAM156S — taux de chômage mensuel CVS, % (source primaire : Enquête sur la population active, StatCan) — Série mensuelle ; 15 ans et plus. | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTCAM156S | 2026-11-04 | 34 obs, 2023-11-01 → 2026-08-01 |
| `real_gdp_growth` | FRED (OECD National Accounts) NAEXKP01CAQ657S — PIB réel par les dépenses, croissance t/t-1 CVS % (source primaire : StatCan) — YoY calculé par capitalisation des 4 taux trimestriels, arrondi à 1 décimale. | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CAQ657S | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |
| `cpi_common` | Banque du Canada — CPI-common, % variation annuelle (API Valet) — Mesure d'inflation fondamentale (modèle à facteur commun). | https://www.bankofcanada.ca/valet/observations/CPI_COMMON/json?start_date=2023-10-01 | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `sp_global_manufacturing_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Canada Manufacturing », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-09-01 → 2026-10-01 |
| `sp_global_services_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « Canada Services », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 1 obs, 2026-09-01 → 2026-09-01 |

## AUD

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Reserve Bank of Australia — Cash Rate Target (table F1.1, série FIRMMCRT, via DBnomics) — Taux en vigueur au 1er jour du mois (dernier changement ≤ au 1er). | https://api.db.nomics.world/v22/series/RBA/F1.1/FIRMMCRT?observations=1&format=json | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `policy_rate` | Reserve Bank of Australia — Cash Rate Target, table officielle des décisions (08/11/23 4,35 ; 19/02/25 4,10 ; 21/05/25 3,85 ; 13/08/25 3,60 ; 04/02/26 3,85 ; 18/03/26 4,10 ; 06/05/26 4,35 ; 30/09/26 4,60 ; 04/11/26 4,85 hors fenêtre) — Taux en vigueur au 1er jour du mois. Croisé avec la série quotidienne FIRMMCRT (table RBA F1.1, via DBnomics). | https://www.rba.gov.au/statistics/cash-rate/ | 2026-11-04 | 〃 |
| `inflation_rate` | Reserve Bank of Australia table G1 — trimmed mean inflation, year-ended %, excluding interest charges and tax changes (via DBnomics) — Série trimestrielle (le trimmed mean/weighted median mensuel de l'ABS n'est accessible que depuis avril 2024 dans les fichiers publiés). | https://api.db.nomics.world/v22/series/RBA/G1/GCPIOCPMTMYP?observations=1&format=json | 2026-11-04 | 12 obs, 2023-12-01 → 2026-09-01 |
| `inflation_rate` | Reserve Bank of Australia table G1 — weighted median inflation, year-ended %, excluding interest charges and tax changes (via DBnomics) — Série trimestrielle (le trimmed mean/weighted median mensuel de l'ABS n'est accessible que depuis avril 2024 dans les fichiers publiés). | https://api.db.nomics.world/v22/series/RBA/G1/GCPIOCPMWMYP?observations=1&format=json | 2026-11-04 | 〃 |
| `ppi_yoy` | ABS 6427.0 tableau 1 — Producer Price Index, Final, Total, % variation an/an (série A2314867K) — L'indice des prix à la production australien est TRIMESTRIEL. | https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/producer-price-indexes-australia/sep-2026/642701.xlsx | 2026-11-04 | 12 obs, 2023-12-01 → 2026-09-01 |
| `unemployment_rate` | ABS 6202.0 tableau 1 — Unemployment rate, Persons, seasonally adjusted, % — Enquête sur la population active, mensuel. | https://www.abs.gov.au/statistics/labour/employment-and-unemployment/labour-force-australia/sep-2026/62020001.xlsx | 2026-11-04 | 35 obs, 2023-11-01 → 2026-09-01 |
| `real_gdp_growth` | ABS 5206.0 Key Aggregates — PIB réel (volume chaîné) : % t/t-1 CVS (value) ; YoY calculé à partir des niveaux CVS (value_secondary) — Dernier point : 2026T2 (le T3 paraît en décembre). | https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product/jun-2026/5206001_Key_Aggregates.xlsx | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |

## NZD

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `policy_rate` | Reserve Bank of New Zealand — Official Cash Rate, table officielle des décisions (14/08/24 5,25 ; 09/10/24 4,75 ; 27/11/24 4,25 ; 19/02/25 3,75 ; 09/04/25 3,50 ; 28/05/25 3,25 ; 20/08/25 3,00 ; 08/10/25 2,50 ; 26/11/25 2,25 ; 08/07/26 2,50 ; 02/09/26 2,75 ; 28/10/26 2,75 hors fenêtre) — Taux en vigueur au 1er jour du mois (le changement s'applique le jour même de la décision). | https://www.rbnz.govt.nz/monetary-policy/monetary-policy-decisions | 2026-11-04 | 36 obs, 2023-11-01 → 2026-10-01 |
| `inflation_rate` | Stats NZ — Consumers Price Index, All groups, % de variation sur le trimestre correspondant de l'année précédente (classeur de publication, Table 1) — Série trimestrielle (l'IPC néo-zélandais est trimestriel). | https://www.stats.govt.nz/information-releases/consumers-price-index-september-2026-quarter | 2026-11-04 | 12 obs, 2023-12-01 → 2026-09-01 |
| `inflation_rate` | Stats NZ — CPI « All groups less food group, household energy subgroup and vehicle fuels », % an/an (Table 3.03, classeurs sep-2024/2025/2026) — Proxy officiel du noyau (hors alimentation et énergies) : les mesures trimmed mean/weighted median de Stats NZ ne sont pas accessibles programmatiquement. | https://www.stats.govt.nz/information-releases/consumers-price-index-september-2026-quarter | 2026-11-04 | 〃 |
| `unemployment_rate` | FRED (OECD Infra-Annual Labor Statistics) LRHUTTTTNZQ156S — taux de chômage trimestriel CVS, % (source primaire : Household Labour Force Survey, Stats NZ) — Série trimestrielle. | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTNZQ156S | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |
| `real_gdp_growth` | FRED (OECD National Accounts) NAEXKP01NZQ657S — PIB réel, croissance t/t-1 CVS % (source primaire : Stats NZ) — YoY calculé par capitalisation des 4 taux trimestriels. Dernier point : 2026T2 (le T3 paraît mi-décembre). | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01NZQ657S | 2026-11-04 | 11 obs, 2023-12-01 → 2026-06-01 |

## CNY

| Série | Définition / institution | URL | Collecté le | Couverture |
|---|---|---|---|---|
| `inflation_rate` | NBS (国家统计局) — communiqués mensuels « 居民消费价格 » : CPI YoY, % (valeur du titre) — CPI national, variation annuelle. Titres des communiqués de presse officiels. | https://www.stats.gov.cn/sj/zxfb/index.html | 2026-11-04 | 32 obs, 2023-11-01 → 2026-09-01 |
| `inflation_rate` | NBS — CPI « 不包括食品和能源 » (hors alimentation et énergie), YoY, % — Extrait du tableau des communiqués mensuels CPI (2e colonne = variation annuelle). | https://www.stats.gov.cn/sj/zxfb/index.html | 2026-11-04 | 〃 |
| `ppi_yoy` | NBS — communiqués mensuels « 工业生产者出厂价格 » : PPI YoY, % — PPI national, variation annuelle (titre ou corps du communiqué). | https://www.stats.gov.cn/sj/zxfb/index.html | 2026-11-04 | 33 obs, 2023-11-01 → 2026-09-01 |
| `unemployment_rate` | NBS — communiqués mensuels « 国民经济运行 » : 全国城镇调查失业率 (taux de chômage urbain enquêté), % — Extrait du corps des communiqués mensuels. | https://www.stats.gov.cn/sj/zxfb/index.html | 2026-11-04 | 9 obs, 2024-04-01 → 2026-08-01 |
| `real_gdp_growth` | NBS — communiqué « 国内生产总值初步核算结果 » : Table 2 (GDP 同比增长速度, YoY) et Table 3 (环比增长速度, QoQ), % — Chaque communiqué trimestriel contient l'historique complet des taux trimestriels. | https://www.stats.gov.cn/sj/zxfb/202610/t20261020_1965658.html | 2026-11-04 | 12 obs, 2023-12-01 → 2026-09-01 |
| `caixin_manufacturing_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « China General Manufacturing », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 1 obs, 2026-10-01 → 2026-10-01 |
| `caixin_services_pmi` | S&P Global / portail PMI (pmi.spglobal.com) — communiqués de presse « China General Services », indice désaisonnalisé — Investing.com inaccessible (HTTP 403) ; le portail S&P Global ne liste que les communiqués récents → couverture partielle (~2-4 mois). Chaque communiqué fournit le mois courant et le mois précédent. | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2026-11-04 | 2 obs, 2026-09-01 → 2026-10-01 |