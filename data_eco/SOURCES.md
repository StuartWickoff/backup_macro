# SOURCES.md — Journal des sources de `data_eco/`

Collecte initiale le **4 novembre 2026** (1ʳᵉ passe) ; corrections et compléments le
**4 novembre 2026** (2ᵉ passe, suite à un audit externe). Chaque valeur provient
exclusivement des sources listées ci-dessous ; aucune valeur n'a été inventée ou
interpolée.

Priorité demandée : 1) Investing.com (valeurs « Actual ») — **inaccessible depuis cet
environnement (HTTP 403, protection anti-robot ; aucun contournement tenté)** ; 2) sources
officielles (banques centrales, instituts statistiques, S&P Global). Les agrégateurs
FRED/OECD, DBnomics et China Data Portal ont servi d'accès techniques vers des séries
officielles lorsque le site primaire bloquait l'accès programmatique — chaque usage est
signalé dans `_fetch_log.json` et, le cas échéant, validé par points de contrôle.

## Conventions (corrigées le 04/11/2026)

- Format : `series_id,date,value,value_secondary` ; dates ISO (`YYYY-MM-DD`).
- **`value_secondary` est vide pour toutes les lignes** : chaque mesure possède son propre
  `series_id` (ex. `hicp_headline_yoy` et `hicp_core_yoy` ; `gdp_qoq` et `gdp_yoy`).
- Séries mensuelles : date = **1ᵉʳ jour du mois** d'observation.
- **Séries trimestrielles : date = 1ᵉʳ jour du trimestre** (T1→`01-01`, T2→`04-01`,
  T3→`07-01`, T4→`10-01`) — convention demandée, corrigée en 2ᵉ passe.
- `policy_rate` : taux **en vigueur au 1ᵉʳ jour du mois** (dernier changement ≤ au 1ᵉʳ).
- `gdp_qoq` : croissance t/t-1 (CVS, non annualisée) ; `gdp_yoy` : croissance an/an
  (publiée par la source ou calculée par capitalisation — signalé dans `_fetch_log.json`).
- Fenêtre cible : 2023-11 → 2026-10 (36 mois) et 2023T4 → 2026T3 (12 trimestres) ; la
  couverture réelle dépend des calendriers de publication — les **dates manquantes exactes**
  et leurs causes sont listées dans `CONTROL_SUMMARY.md`.

## Zone euro — agrégats EA20 / EA21

La Bulgarie a rejoint la zone euro le **1ᵉʳ janvier 2026** (passage EA20 → EA21).
Eurostat publie les deux géos (EA20 : composition 2023-2025 ; EA21 : composition 2026,
rétropolée) ; l'écart entre agrégats est marginal. Convention retenue, série par série :

| Série | Géo | Note |
|---|---|---|
| `hicp_headline_yoy`, `hicp_core_yoy` | EA20 | Contrôle EA21 : valeurs identiques sauf 3-4 mois à ±0,1 pt ; flash oct. 2026 (4,0 / 2,5) = communiqué Eurostat du 04/11/2026 |
| `unemployment_rate` | EA21 | L'agrégat EA20 n'est pas publié pour cette série (une seule définition) |
| `ppi_yoy` | EA21 | Série intégrale vérifiée observation par observation (EA20 : ≤0,1 pt d'écart) |
| `gdp_qoq`, `gdp_yoy` | EA20 | T3 2026 = estimation flash du 30/10/2026 |

## Chine — PMI « RatingDog » (ex-Caixin)

L'indice historiquement appelé **Caixin China General PMI** est publié sous la marque
**« RatingDog China »** depuis août 2025 (changement de sponsor ; l'enquête reste compilée
et publiée par S&P Global, méthodologie inchangée). Les identifiants
`caixin_manufacturing_pmi` / `caixin_services_pmi` ont été renommés
`ratingdog_manufacturing_pmi` / `ratingdog_services_pmi` ; la continuité de la série est
totale.

## Suivi des révisions / statuts provisoires

- **EUR** : HICP octobre 2026 = estimation **flash** (4,0 / 2,5 — définitif le 18/11/2026) ;
  PIB T3 2026 = estimation flash (30/10/2026).
- **PMI** : seules les valeurs **finales** sont utilisées (ex. manufacturing zone euro
  oct. : flash 52,4 → final 52,7).
- **RBA** : relèvement à 4,85 % le 04/11/2026 — postérieur à l'observation du 1ᵉʳ octobre
  (4,60 %) ; apparaîtra dans la fenêtre à la prochaine actualisation.
- **PBOC** : l'ancre de la politique monétaire est passée de la pension inversée 7 jours à
  la pension inversée **au jour le jour (1,25 %)** le 02/11/2026 (hors fenêtre) ;
  `policy_rate` (7 jours) = 1,40 % jusqu'à octobre 2026 inclus.


## EUR — Zone euro

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | BCE — Deposit facility rate, niveau au 1ᵉʳ jour du mois | https://data-api.ecb.europa.eu/service/data/FM/B.U2.EUR.4F.KR.DFR.LEV?format=csvdata&startPeriod=2023-08-01 | 36 obs (mensuel), 2023-11 → 2026-10 |
| `hicp_headline_yoy` | Eurostat ei_cphi_m — HICP total, % an/an, EA20 | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=EA20&indic=TOTAL&unit=RT12&sinceTimePeriod=2023-11 | 36 obs (mensuel), 2023-11 → 2026-10 |
| `hicp_core_yoy` | Eurostat ei_cphi_m — HICP hors énergie, aliments, alcool et tabac, % an/an, EA20 | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ei_cphi_m?format=JSON&lang=EN&geo=EA20&indic=CP-HI00XEF&unit=RT12&sinceTimePeriod=2023-11 | 36 obs (mensuel), 2023-11 → 2026-10 |
| `ppi_yoy` | Eurostat sts_inpp_m — prix industriels à la production (B-D, NSA), % an/an, EA21 | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/sts_inpp_m?format=JSON&lang=EN&geo=EA21&nace_r2=B-D&s_adj=NSA&unit=PCH_SM&sinceTimePeriod=2023-11 | 34 obs (mensuel), 2023-11 → 2026-08 |
| `unemployment_rate` | Eurostat une_rt_m — taux de chômage, % population active, CVS, EA21 | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/une_rt_m?format=JSON&lang=EN&geo=EA21&age=TOTAL&sex=T&unit=PC_ACT&s_adj=SA&sinceTimePeriod=2023-11 | 35 obs (mensuel), 2023-11 → 2026-09 |
| `gdp_qoq` | Eurostat namq_10_gdp — PIB réel B1GQ, t/t-1 CVS %, EA20 (T3 2026 : flash) | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/namq_10_gdp?format=JSON&lang=EN&geo=EA20&na_item=B1GQ&unit=CLV_PCH_PRE&sinceTimePeriod=2023-Q4 | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `gdp_yoy` | Eurostat namq_10_gdp — PIB réel B1GQ, an/an %, EA20 | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/namq_10_gdp?format=JSON&lang=EN&geo=EA20&na_item=B1GQ&unit=CLV_PCH_SM&sinceTimePeriod=2023-Q4 | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `sp_global_manufacturing_pmi` | S&P Global PMI — Eurozone Manufacturing, indice final désaisonnalisé (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `sp_global_services_pmi` | S&P Global PMI — Eurozone Services Business Activity Index (cité dans les communiqués Composite) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 3 obs (mensuel), 2026-08 → 2026-10 |

## GBP — Royaume-Uni

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | Bank of England — Official Bank Rate, taux au 1ᵉʳ jour du mois | https://www.bankofengland.co.uk/boeapps/database/Bank-Rate.asp | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_headline_yoy` | ONS MM23 — CPI annual rate, all items (D7G7) | https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/d7g7/data?format=csv | 35 obs (mensuel), 2023-11 → 2026-09 |
| `cpi_core_yoy` | ONS MM23 — CPI 12 mois hors énergie, aliments, boissons alcoolisées et tabac | https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceindices | 35 obs (mensuel), 2023-11 → 2026-09 |
| `ppi_yoy` | ONS — PPI output prices, all manufactured products (GB7S), YoY calculé sur l'indice 2015=100 | https://www.ons.gov.uk/economy/inflationandpriceindices/timeseries/gb7s/data?format=csv | 35 obs (mensuel), 2023-11 → 2026-09 |
| `unemployment_rate` | ONS MGSX — taux de chômage (16 ans et plus, CVS), EPA | https://www.ons.gov.uk/employmentandlabourmarket/peoplenotinwork/unemployment/timeseries/mgsx/data?format=csv | 33 obs (mensuel), 2023-11 → 2026-07 |
| `gdp_qoq` | ONS IHYQ — GDP quarter on quarter growth, CVM SA % | https://www.ons.gov.uk/economy/grossdomesticproductgdp/timeseries/ihyq/data?format=csv | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | ONS IHYP — GDP year on year growth, CVM SA % | https://www.ons.gov.uk/economy/grossdomesticproductgdp/timeseries/ihyp/data?format=csv | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `sp_global_manufacturing_pmi` | S&P Global PMI — UK Manufacturing (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `sp_global_services_pmi` | S&P Global PMI — UK Services (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |

## JPY — Japon

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | Bank of Japan — taux directeur (communiqués MPM), taux au 1ᵉʳ jour du mois | https://www.boj.or.jp/en/mopo/mpmdeci/index.htm | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_headline_yoy` | Statistics Bureau of Japan — CPI all items (STATJP/CPIm 001 via DBnomics), YoY calculé | https://api.db.nomics.world/v22/series/STATJP/CPIm/001?observations=1&format=json | 35 obs (mensuel), 2023-11 → 2026-09 |
| `cpi_core_yoy` | Statistics Bureau of Japan — CPI all items, less fresh food (CPIm 733 via DBnomics), YoY calculé — « core » officiel japonais | https://api.db.nomics.world/v22/series/STATJP/CPIm/733?observations=1&format=json | 35 obs (mensuel), 2023-11 → 2026-09 |
| `ppi_yoy` | Bank of Japan — Corporate Goods Price Index, all commodities, % an/an (PDF mensuels) | https://www.boj.or.jp/en/statistics/pi/cgpi_release/index.htm | 35 obs (mensuel), 2023-11 → 2026-09 |
| `unemployment_rate` | Statistics Bureau of Japan — taux de chômage mensuel, CVS (STATJP/MIm via DBnomics) | https://api.db.nomics.world/v22/series/STATJP/MIm/M.UR.B.PCT.SA?observations=1&format=json | 35 obs (mensuel), 2023-11 → 2026-09 |
| `gdp_qoq` | Cabinet Office ESRI — Quarterly Estimates of GDP, t/t-1 CVS (ritu-jk, T2 2026 2ᵉ estim.) | https://www.esri.cao.go.jp/en/sna/data/sokuhou/files/toukei_top.html | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | Cabinet Office ESRI — ritu-jg (an/an) | https://www.esri.cao.go.jp/en/sna/data/sokuhou/files/toukei_top.html | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `sp_global_manufacturing_pmi` | S&P Global PMI (au Jibun Bank) — Japan Manufacturing (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `sp_global_services_pmi` | S&P Global PMI (au Jibun Bank) — Japan Services (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-08 → 2026-09 |

## CHF — Suisse

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | Banque nationale suisse — taux directeur (portail de données, cube snbgwdzid), taux au 1ᵉʳ jour du mois | https://data.snb.ch/api/cube/snbgwdzid/data/csv/en | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_headline_yoy` | OFS — IPC national (Landesindex), total, glissement annuel, base déc. 2025=100 (fichier maître du 03/11/2026) — remplace l'HICP (2ᵉ passe, conformément à la demande initiale) | https://www.bfs.admin.ch/en/newnsb/UA8hylpLYO6i | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_core_yoy` | OFS — « Kerninflation 1 » : IPC national hors produits frais et saisonniers, énergie et carburants | https://www.bfs.admin.ch/en/newnsb/UA8hylpLYO6i | 36 obs (mensuel), 2023-11 → 2026-10 |
| `ppi_yoy` | Eurostat sts_inpp_m geo=CH — prix à la production (B-D), % an/an (source primaire : OFS) | https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/sts_inpp_m?format=JSON&lang=EN&geo=CH&nace_r2=B-D&s_adj=NSA&unit=PCH_SM&sinceTimePeriod=2023-11 | 35 obs (mensuel), 2023-11 → 2026-09 |
| `unemployment_rate` | FRED (OECD IALS) LRHUTTTTCHQ156S — taux de chômage trimestriel CVS, définition BIT (source primaire : OFS/SECO) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTCHQ156S | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_qoq` | FRED (OECD) NAEXKP01CHQ657S — PIB réel t/t-1 CVS % (source primaire : SECO ; contrôle croisé SECO/DBnomics) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CHQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | YoY calculé par capitalisation des 4 taux trimestriels (FRED NAEXKP01CHQ657S) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CHQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |

*PMI suisses (SVME/procure.ch, services) : **non collectés** — aucune source programmatique accessible (portail S&P Global sans communiqués suisses, Investing.com bloqué). Dernière valeur connue au 04/11/2026 : manufacturing oct. = 56,7 (sept. 55,3), publication SVME du 02/11/2026.*

## CAD — Canada

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | Banque du Canada — cible du taux de financement à un jour (API Valet, CBC20210), taux au 1ᵉʳ jour du mois | https://www.bankofcanada.ca/valet/observations/CBC20210/json?start_date=2023-10-01 | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_trimmed_yoy` | Banque du Canada — CPI-trim, % an/an (API Valet CPI_TRIM) | https://www.bankofcanada.ca/valet/observations/CPI_TRIM/json?start_date=2023-10-01 | 35 obs (mensuel), 2023-11 → 2026-09 |
| `cpi_median_yoy` | Banque du Canada — CPI-median, % an/an (API Valet CPI_MEDIAN) | https://www.bankofcanada.ca/valet/observations/CPI_MEDIAN/json?start_date=2023-10-01 | 35 obs (mensuel), 2023-11 → 2026-09 |
| `cpi_common` | Banque du Canada — CPI-common, % an/an (API Valet CPI_COMMON) | https://www.bankofcanada.ca/valet/observations/CPI_COMMON/json?start_date=2023-10-01 | 35 obs (mensuel), 2023-11 → 2026-09 |
| `unemployment_rate` | FRED (OECD IALS) LRHUTTTTCAM156S (source primaire : StatCan, EPA) ; sept. 2026 ajouté depuis StatCan (LFS du 09/10/2026) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTCAM156S | 35 obs (mensuel), 2023-11 → 2026-09 |
| `gdp_qoq` | FRED (OECD) NAEXKP01CAQ657S — PIB réel par les dépenses t/t-1 CVS % (source primaire : StatCan) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CAQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | YoY calculé par capitalisation des 4 taux trimestriels (FRED NAEXKP01CAQ657S) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01CAQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `sp_global_manufacturing_pmi` | S&P Global PMI — Canada Manufacturing (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `sp_global_services_pmi` | S&P Global PMI — Canada Services (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 3 obs (mensuel), 2026-08 → 2026-10 |
| `ppi_yoy` | Statistics Canada — IPPI total, variation annuelle, non désaisonnalisée ; 2026-09 = +14,5 % (The Daily, 22/10/2026). Historique mensuel non reconstitué dans ce CSV. | https://www150.statcan.gc.ca/n1/daily-quotidien/261022/dq261022a-eng.htm ; tableau 18-10-0265-02 : https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810026502 | 1 obs (mensuel), 2026-09 |

*`ppi_yoy` (IPPI) : **historique partiel** — le point officiel de septembre 2026 (+14,5 % an/an) a été ajouté. Les 35 autres mois de la fenêtre n'ont pas été importés ; ils restent manquants et ne sont pas estimés.*

## AUD — Australie

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | RBA — Cash Rate Target (table F1.1 FIRMMCRT via DBnomics + table officielle des décisions), taux au 1ᵉʳ jour du mois | https://www.rba.gov.au/statistics/cash-rate/ | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_trimmed_yoy` | RBA table G1 — trimmed mean, % an/an (via DBnomics) — série trimestrielle | https://api.db.nomics.world/v22/series/RBA/G1/GCPIOCPMTMYP?observations=1&format=json | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `cpi_weighted_median_yoy` | RBA table G1 — weighted median, % an/an (via DBnomics) — série trimestrielle | https://api.db.nomics.world/v22/series/RBA/G1/GCPIOCPMWMYP?observations=1&format=json | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `ppi_yoy` | ABS 6427.0 tableau 1 — Producer Price Index (final demand), % an/an — série trimestrielle | https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/producer-price-indexes-australia/sep-2026/642701.xlsx | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `unemployment_rate` | ABS 6202.0 tableau 1 — taux de chômage, personnes, CVS | https://www.abs.gov.au/statistics/labour/employment-and-unemployment/labour-force-australia/sep-2026/62020001.xlsx | 35 obs (mensuel), 2023-11 → 2026-09 |
| `gdp_qoq` | ABS 5206.0 Key Aggregates — PIB réel (volume chaîné) t/t-1 CVS % | https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product/jun-2026/5206001_Key_Aggregates.xlsx | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | YoY calculé à partir des niveaux CVS (ABS 5206.0) | https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product/jun-2026/5206001_Key_Aggregates.xlsx | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `sp_global_manufacturing_pmi` | S&P Global PMI — Australia Manufacturing (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `sp_global_services_pmi` | S&P Global PMI — Australia Services (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |

## NZD — Nouvelle-Zélande

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | RBNZ — Official Cash Rate, table des décisions, taux au 1ᵉʳ jour du mois | https://www.rbnz.govt.nz/monetary-policy/monetary-policy-decisions | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_headline_yoy` | Stats NZ — Consumers Price Index, all groups, % an/an (Table 1) — série trimestrielle | https://www.stats.govt.nz/information-releases/consumers-price-index-september-2026-quarter | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `cpi_core_proxy_yoy` | Stats NZ — CPI « all groups less food group, household energy subgroup and vehicle fuels » (Table 3.03) — proxy officiel du noyau, pas une mesure universelle du « core » | https://www.stats.govt.nz/information-releases/consumers-price-index-september-2026-quarter | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `ppi_yoy` | Stats NZ — Business Price Indexes, PPI sorties toutes industries (SQU900000), % an/an (Table 1.03 des classeurs jun-2026, mar-2025, dec-2024) — ajouté en 2ᵉ passe | https://www.stats.govt.nz/information-releases/business-price-indexes-june-2026-quarter | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `unemployment_rate` | FRED (OECD IALS) LRHUTTTTNZQ156S (source primaire : Stats NZ, HLFS) ; T3 2026 ajouté depuis Stats NZ (LMS du 04/11/2026) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=LRHUTTTTNZQ156S | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `gdp_qoq` | FRED (OECD) NAEXKP01NZQ657S — PIB réel t/t-1 CVS % (source primaire : Stats NZ) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01NZQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |
| `gdp_yoy` | YoY calculé par capitalisation des 4 taux trimestriels (FRED NAEXKP01NZQ657S) | https://fred.stlouisfed.org/graph/fredgraph.csv?id=NAEXKP01NZQ657S | 11 obs (trimestriel), 2023-10 → 2026-04 |

*PMI néo-zélandais (BNZ-BusinessNZ PMI/PSI) : **non collectés** — sites BusinessNZ/BNZ inaccessibles (HTTP 403), Investing.com bloqué.*

## CNY — Chine

| Série | Définition / institution | URL | Couverture |
|---|---|---|---|
| `policy_rate` | PBOC — taux de prise en pension inversée à 7 jours, taux en vigueur au 1ᵉʳ jour du mois ; série reconstituée à partir des annonces de taux de la banque centrale relayées par le Conseil des affaires d'État (Xinhua) | [22/07/2024](https://english.www.gov.cn/news/202407/22/content_WS669dc04ec6d0868f4e8e9543.html) ; [27/09/2024](https://english.www.gov.cn/news/202409/27/content_WS66f618efc6d0868f4e8eb514.html) ; [07/05/2025](https://english.www.gov.cn/news/202505/07/content_WS681af03ec6d0868f4e8f250c.html) | 36 obs (mensuel), 2023-11 → 2026-10 |
| `cpi_headline_yoy` | NBS — communiqués mensuels « 居民消费价格 » : CPI YoY, % (titre ou corps du communiqué) | https://www.stats.gov.cn/sj/zxfb/ | 35 obs (mensuel), 2023-11 → 2026-09 |
| `cpi_core_yoy` | NBS — CPI « 不包括食品和能源 » (hors alimentation et énergie), YoY, % (tableau du communiqué) | https://www.stats.gov.cn/sj/zxfb/ | 35 obs (mensuel), 2023-11 → 2026-09 |
| `ppi_yoy` | NBS — communiqués mensuels « 工业生产者出厂价格 » : PPI YoY, % | https://www.stats.gov.cn/sj/zxfb/ | 35 obs (mensuel), 2023-11 → 2026-09 |
| `unemployment_rate` | China Data Portal (chinadata.live) — miroir du taux de chômage urbain enquêté NBS (validé sur 11 points de contrôle) ; portail de données NBS inaccessible (403) | https://chinadata.live/api/v2/data/china-unemployment | 35 obs (mensuel), 2023-11 → 2026-09 |
| `gdp_qoq` | NBS — communiqué trimestriel « 国内生产总值初步核算结果 » : Table 3 环比 (QoQ), % | https://www.stats.gov.cn/sj/zxfb/202610/t20261020_1965658.html | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `gdp_yoy` | NBS — communiqué trimestriel : Table 2 同比 (YoY), % | https://www.stats.gov.cn/sj/zxfb/202610/t20261020_1965658.html | 12 obs (trimestriel), 2023-10 → 2026-07 |
| `ratingdog_manufacturing_pmi` | S&P Global / RatingDog China (ex-Caixin) — General Manufacturing PMI (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
| `ratingdog_services_pmi` | S&P Global / RatingDog China (ex-Caixin) — General Services Business Activity Index (communiqués) | https://www.pmi.spglobal.com/Public/Release/PressReleases | 2 obs (mensuel), 2026-09 → 2026-10 |
