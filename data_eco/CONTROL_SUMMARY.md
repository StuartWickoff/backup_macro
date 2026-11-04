# CONTROL_SUMMARY.md — Résumé de contrôle

Généré le **4 novembre 2026** (2ᵉ passe, après correction). Fenêtre cible : 2023-11 →
2026-10 (36 mois) / 2023T4 → 2026T3 (12 trimestres). **Toutes les dates manquantes sont
listées avec leur cause** (calendrier de publication au 04/11/2026 ou indisponibilité).
Total : **1429 observations** (1 011 avant correction) sur 8 devises.
Contrôle automatisé : exécuter `python3 audit.py` dans ce répertoire (rapport `AUDIT.md`).

## Corrections appliquées le 04/11/2026 (2ᵉ passe)

**P0 — schéma**
1. **Éclatement de toutes les mesures** : `inflation_rate` → deux séries par devise
   (`hicp_headline_yoy`/`hicp_core_yoy` pour EUR et CHF ; `cpi_headline_yoy`/`cpi_core_yoy`
   pour GBP, JPY, CNY ; `cpi_trimmed_yoy`/`cpi_median_yoy` pour CAD ;
   `cpi_trimmed_yoy`/`cpi_weighted_median_yoy` pour AUD ;
   `cpi_headline_yoy`/`cpi_core_proxy_yoy` pour NZD) ; `real_gdp_growth` → `gdp_qoq` +
   `gdp_yoy`. **`value_secondary` est vide sur les 1 429 lignes.**
2. **Dates trimestrielles corrigées** : 1ᵉʳ jour du trimestre (T1→01-01, T2→04-01,
   T3→07-01, T4→10-01) — 148 lignes re-datées (PIB, IPC/PPI trimestriels australiens et
   néo-zélandais, chômage trimestriel suisse et néo-zélandais).
3. **Réconciliation journal ↔ CSV (PMI AUD)** : les observations notées dans
   `_fetch_log.json` mais absentes du CSV ont été ajoutées depuis les communiqués
   S&P Global déjà téléchargés (manufacturing 49,6 sept. et oct. ; services 51,9 sept.,
   48,5 oct.). Le journal a été consolidé (une entrée par série, cohérente avec les CSV).

**P1 — données**
4. **Chômage Chine** : série complète 35 mois (nov. 2023 → sept. 2026) via le miroir NBS
   chinadata.live, validée sur 11 points de contrôle (9 communiqués NBS déjà collectés +
   sept. 5,2 % / août 5,3 % vérifiés) — auparavant 9 observations éparses.
5. **Taux PBOC (pension inversée 7 jours) ajouté** : 36 mois, reconstitué depuis les
   annonces vérifiées (1,80 % → 1,70 % le 22/07/24 → 1,50 % le 27/09/24 → 1,40 % le
   07/05/25, inchangé depuis ; corroboration LPR/BIS, Reuters 28/06/26, CEIC oct. 26).
6. **PMI** : services zone euro créés (3 obs : 51,6 / 53,0 / 51,4) ; manufacturing zone
   euro sept. ajouté (52,9) ; services Canada août + oct. ajoutés (46,8 / 48,1) ;
   RatingDog manufacturing sept. ajouté (52,1).
7. **Correction de valeur** : PMI services Japon août 2026 = 52,5 (l'ancienne valeur 53,5
   était l'indice **composite**, pas le services).
8. **Chômage** : Canada sept. 2026 = 6,5 % (StatCan 09/10) ; Nouvelle-Zélande T3 2026 =
   5,7 % (Stats NZ 04/11) ajoutés.
9. **PPI Nouvelle-Zélande ajouté** : 11 trimestres (PPI sorties, toutes industries).
10. **Suisse — retour à l'IPC national** (demande initiale) : `cpi_headline_yoy` (LIK,
    36 mois jusqu'à oct. 2026 : +1,5 %) et `cpi_core_yoy` (Kerninflation 1, 36 mois)
    remplacent les séries HICP (disponibles dans l'historique Git).
11. **Chine — trous comblés** : CPI janv. 2024 (−0,8), juil. 2024 (+0,5), juil. 2025 (0,0) ;
    core janv. 2024, juil. 2024, janv. 2025, juil. 2025, janv. 2026 ; PPI déc. 2024 (−2,3),
    mars 2026 (+0,5).
12. **Renommage** `caixin_*` → `ratingdog_*` (continuité Caixin → RatingDog documentée).
13. **EA20/EA21 normalisé et documenté** (voir SOURCES.md) ; la double définition du PPI
    zone euro est levée (série EA21 intégrale).

**P2 — traçabilité** : URLs directes par série, statuts flash/provisoires, liste exacte
des dates manquantes (ci-dessous).


## EUR — Zone euro

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `gdp_qoq` | 12 | 2026-07 | — | complet |
| `gdp_yoy` | 12 | 2026-07 | — | complet |
| `hicp_core_yoy` | 36 | 2026-10 | — | complet |
| `hicp_headline_yoy` | 36 | 2026-10 | — | complet |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 34 | 2026-08 | 2026-09, 2026-10 | sept. : publication Eurostat du 04/11 non encore parvenue dans l'API lors de la 2ᵉ passe ; oct. : prévu début déc. 2026 |
| `sp_global_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | portail S&P Global ≈ 2 mois de communiqués ; Investing.com 403 → historique non reconstituable |
| `sp_global_services_pmi` | 3 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07 | idem manufacturing |
| `unemployment_rate` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication Eurostat prévue début déc. 2026 |

## GBP — Royaume-Uni

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_core_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication ONS du 18/11/2026 |
| `cpi_headline_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication ONS du 18/11/2026 |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : 1ʳᵉ estimation ONS prévue ~13/11/2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : 1ʳᵉ estimation ONS prévue ~13/11/2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication ONS du 18/11/2026 |
| `sp_global_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | historique PMI non reconstituable (portail ≈ 2 mois ; Investing.com 403) |
| `sp_global_services_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | idem |
| `unemployment_rate` | 33 | 2026-07 | 2026-08, 2026-09, 2026-10 | août 2026 : prochaine publication LMS ~10/11/2026 ; sept. ~10/12/2026 ; oct. ~12/01/2027 |

## JPY — Japon

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_core_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication du ~20/11/2026 |
| `cpi_headline_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication du ~20/11/2026 |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : 1ʳᵉˢ estimations ESRI prévues ~16/11/2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : 1ʳᵉˢ estimations ESRI prévues ~16/11/2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication BOJ (CGPI) du ~13/11/2026 |
| `sp_global_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | historique PMI non reconstituable |
| `sp_global_services_pmi` | 2 | 2026-09 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-10 | oct. 2026 non publié au 04/11 (communiqué attendu) + historique non reconstituable |
| `unemployment_rate` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication du début déc. 2026 |

## CHF — Suisse

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_core_yoy` | 36 | 2026-10 | — | complet |
| `cpi_headline_yoy` | 36 | 2026-10 | — | complet |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication SECO prévue début déc. 2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication SECO prévue début déc. 2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication OFS prévue ~26/11/2026 |
| `unemployment_rate` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication OFS/SAKE prévue mi-nov. 2026 |

## CAD — Canada

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_common` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication StatCan du 17/11/2026 |
| `cpi_median_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication StatCan du 17/11/2026 |
| `cpi_trimmed_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication StatCan du 17/11/2026 |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication StatCan prévue ~26/11/2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication StatCan prévue ~26/11/2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `sp_global_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | historique PMI non reconstituable |
| `sp_global_services_pmi` | 3 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07 | idem |
| `unemployment_rate` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication StatCan (EPA) du 06/11/2026 |

## AUD — Australie

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_trimmed_yoy` | 12 | 2026-07 | — | complet |
| `cpi_weighted_median_yoy` | 12 | 2026-07 | — | complet |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication ABS prévue le 02/12/2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication ABS prévue le 02/12/2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 12 | 2026-07 | — | complet |
| `sp_global_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | historique PMI non reconstituable |
| `sp_global_services_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | idem |
| `unemployment_rate` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication ABS du 19/11/2026 |

## NZD — Nouvelle-Zélande

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_core_proxy_yoy` | 12 | 2026-07 | — | complet |
| `cpi_headline_yoy` | 12 | 2026-07 | — | complet |
| `gdp_qoq` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication Stats NZ prévue ~16/12/2026 |
| `gdp_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication Stats NZ prévue ~16/12/2026 |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 11 | 2026-04 | 2026-T3 | T3 2026 : publication Stats NZ prévue ~17/11/2026 |
| `unemployment_rate` | 12 | 2026-07 | — | complet |

## CNY — Chine

| Série | Obs. | Dernière | Manquant (dates exactes) | Cause |
|---|---|---|---|---|
| `cpi_core_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication NBS prévue ~09/11/2026 |
| `cpi_headline_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication NBS prévue ~09/11/2026 |
| `gdp_qoq` | 12 | 2026-07 | — | complet |
| `gdp_yoy` | 12 | 2026-07 | — | complet |
| `policy_rate` | 36 | 2026-10 | — | complet |
| `ppi_yoy` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication NBS prévue ~09/11/2026 |
| `ratingdog_manufacturing_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | historique PMI non reconstituable |
| `ratingdog_services_pmi` | 2 | 2026-10 | 2023-11, 2023-12, 2024-01, 2024-02, 2024-03, 2024-04, 2024-05, 2024-06, 2024-07, 2024-08, 2024-09, 2024-10, 2024-11, 2024-12, 2025-01, 2025-02, 2025-03, 2025-04, 2025-05, 2025-06, 2025-07, 2025-08, 2025-09, 2025-10, 2025-11, 2025-12, 2026-01, 2026-02, 2026-03, 2026-04, 2026-05, 2026-06, 2026-07, 2026-08 | idem |
| `unemployment_rate` | 35 | 2026-09 | 2026-10 | oct. 2026 : publication NBS prévue ~16/11/2026 |

## Séries indisponibles (identifiées explicitement)

| Devise | Série | Cause |
|---|---|---|
| CHF | PMI manufacturing (SVME/procure.ch) et PMI services | aucune source programmatique accessible (portail S&P Global sans communiqués suisses ; Investing.com 403). Dernières valeurs connues : manufacturing oct. 56,7 / sept. 55,3 (SVME 02/11/2026) |
| NZD | PMI manufacturing et services (BNZ-BusinessNZ) | sites BusinessNZ / BNZ inaccessibles (HTTP 403) ; Investing.com 403 |
| CAD | `ppi_yoy` (IPPI) | StatCan injoignable depuis cet environnement (timeout réseau) ; miroirs FRED/DBnomics/OECD sans équivalent. Dernière valeur connue : sept. 2026 = +14,5 % an/an (The Daily 22/10/2026) |
| toutes | PMI — historique complet 36 mois | le portail S&P Global ne liste que ~2 mois de communiqués ; Investing.com (source prioritaire) inaccessible (HTTP 403) — couverture PMI limitée à 2-3 mois par série |

## Points de contrôle externes (vérifiés le 04/11/2026)

| Point | Valeur fichier | Référence externe |
|---|---|---|
| EUR HICP oct. 2026 (flash) | 4,0 / 2,5 | Eurostat, flash du 04/11/2026 |
| GBP CPI sept. 2026 | 3,5 / 2,8 | ONS, bulletin du 21/10/2026 |
| CNY CPI/PPI sept. 2026 | 0,9 / 1,0 / 5,0 | NBS |
| CNY chômage sept. 2026 | 5,2 | NBS (via miroir validé) |
| CNY PIB T3 2026 | 1,2 QoQ / 4,5 YoY | NBS |
| CHF IPC oct. 2026 | 1,5 (core 0,5) | OFS, communiqué du 03/11/2026 |
| AUD taux directeur 1ᵉʳ oct. | 4,60 | RBA (relèvement à 4,85 % le 04/11, hors fenêtre) |
| CAD chômage sept. 2026 | 6,5 | StatCan LFS 09/10/2026 |
| NZD chômage T3 2026 | 5,7 | Stats NZ LMS 04/11/2026 |
| CNY taux PBOC oct. 2026 | 1,40 | PBOC (inchangé — Reuters 28/06/26, CEIC) |

