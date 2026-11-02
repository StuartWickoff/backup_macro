# COT_ENGINE — Audit du moteur Python COT et contrat d'échange avec Rust

> **Statut** : établi le 2026-11-02 à la demande de l'addendum COT, **avant toute
> modification de code** ; **révisé le 2026-11-03** à l'issue de l'audit transversal de la
> documentation (voir `DOCUMENTATION_AUDIT_REPORT.md` et `SPECIFICATION_DECISIONS.md`).
> Les deux scripts existants ont été examinés intégralement (copies verbatim de référence :
> `work/cot-engine-reference/script_A_dashboard.py` et `script_B_debug_variant.py`).
> **Rôle** : ce document spécifie (1) ce que le moteur Python fait déjà, (2) ses défauts
> confirmés, (3) les corrections prévues avant intégration, (4) le format d'échange
> JSON entre le moteur et le backend Rust, (5) les règles d'import côté Rust.
> **Principe directeur** : le COT est un **indicateur de positionnement, jamais un signal
> autonome** (REQUIREMENTS H8, permanent).

---

## 1. Périmètre et acteurs

| Acteur | Rôle | Statut |
|---|---|---|
| **CFTC** | Fournisseur externe des rapports bruts (Legacy Futures, publication le vendredi pour des positions arrêtées le mardi) | Externe, jamais répliqué |
| **Moteur Python** [OUTIL PERSO] | Téléchargement (`cot_reports`), extraction des 8 marchés suivis, normalisation, calcul des mesures, export | Existant, à durcir (§5) ; **indépendant de l'application** (pas de dépendance WASM, pas d'orchestration par le backend) |
| **Application Rust** | Stockage SQLite structuré des observations normalisées, consultation, liaison aux dossiers devise / paires / analyses | v0.6 |

Chaîne complète, sans exception :

```
CFTC (archives publiques)
  → moteur Python [exécution manuelle, à la demande]
  → fichier JSON (contrat d'échange v1, §6)
  → import manuel dans l'application (POST /api/cot/imports, §7)
  → validation intégrale puis transaction SQLite unique (§7)
  → consultation par le frontend Leptos (dossiers devise, paires, Settings)
```

Formule centrale, inchangée :

```
Net Positioning % OI = (Long Noncommercial - Short Noncommercial) / Open Interest × 100
```

## 2. Inventaire des scripts existants

| | Script A (`script_A_dashboard.py`) | Script B (`script_B_debug_variant.py`) |
|---|---|---|
| Rôle | Générateur complet du dashboard PNG | Variante de **débogage** (écrite pour diagnostiquer « 0 devises trouvées ») ; s'arrête après le résumé console |
| Sélection marchés | Égalité **exacte** sur le nom CFTC complet (`str.strip() ==`, l. 65) | Sous-chaîne + regex (`str.contains`, l. 46) — permissif |
| Devises | 8 : AUD, NZD, EUR, GBP, JPY, CHF, CAD + **USD via 'U.S. DOLLAR INDEX - ICE FUTURES U.S.'** (l. 30) | 7 (USD absent) ; NZD en regex `'NEW ZEALAND DOLLAR|NZ DOLLAR'` (l. 13) |
| Colonnes utilisées | En-têtes CFTC officiels : `Market and Exchange Names`, `As of Date in Form YYYY-MM-DD`, `Noncommercial Positions-Long (All)`, `Noncommercial Positions-Short (All)`, `Open Interest (All)` | Variantes soulignées : `Market_and_Exchange_Names`, `Report_Date_as_MM_DD_YYYY`, `NonComm_Positions_Long_All`, `NonComm_Positions_Short_All`, `Open_Interest_All` |
| Percentiles | `tail(52).quantile()` au moment du téléchargement, lignes droites sur tout l'historique (l. 84-87) | Idem (l. 61-64), commenté « Style ForexFondamentale : lignes droites » |
| Sortie | `outputs/cot_dashboard.png` (matplotlib, thème sombre) + console | Console + prints de debug (colonnes, noms de marchés) |
| Seuil d'inclusion | `len(data) > 10` | `len(data) > 5` |
| Imports morts | — | `matplotlib`, `gridspec` importés jamais utilisés (l. 3-4) |

Les deux scripts correspondent à deux conventions de colonnes du paquet `cot_reports`
(dérive de version) — d'où la coexistence de deux styles d'en-têtes.

## 3. Ce que le moteur fait déjà (à conserver)

1. Téléchargement des rapports annuels Legacy Futures (`cot.cot_year`, try/except par
   année).
2. Extraction des marchés suivis (8 devises, USD inclus côté script A).
3. Récupération des positions **Noncommercial Long / Short** (les spéculateurs — ce qui
   intéresse la méthode, pas les commercials de couverture).
4. Récupération de l'**Open Interest**.
5. Calcul du **positionnement net** (Long − Short) — précision : jamais materialisé
   séparément dans les scripts, il n'apparaît qu'au numérateur du % OI ; il le sera dans
   l'export JSON.
6. **Normalisation par l'OI** (% OI) — rend les marchés comparables entre eux.
7. Calcul de **percentiles historiques** (P20/P80) — idée correcte, implémentation biaisée
   (§4.1).
8. Génération de graphiques (dashboard 2 colonnes, thème sombre, zone neutre, signal
   visuel sur la dernière valeur, annotations).

## 4. Défauts confirmés (audit intégral — lecture du code, sans exécution)

### 4.1 Biais temporel des percentiles — le défaut majeur

`val_20 = sub['Net_Pct_OI'].tail(ROLLING_WINDOW).quantile(PCT_LOW)` (A l. 84-85, B l. 62) :
les percentiles sont calculés sur les **52 dernières observations à la date du
téléchargement**, puis affectés comme constantes (`sub['Pct_20'] = val_20`) à **tout
l'historique**. Conséquences :

- **Look-ahead** : un point de 2021 est comparé à des percentiles calculés avec des
  données de 2026 — l'historique affiché n'est pas reconstituable « comme vu à l'époque » ;
- **Lignes droites** : les bornes ne se déplacent pas au fil du temps, contrairement à ce
  qu'un percentile glissant ferait ;
- Toute nouvelle exécution **réécrit implicitement l'historique** des bornes (sans
  journalisation).

**Correction prévue (§5.1)** : percentile glissant « as-of » — pour chaque observation à la
date t, fenêtre des 52 observations **se terminant à t inclus** (la valeur de t est connue
au moment de la publication du vendredi).

### 4.2 Filtrage trop permissif (script B)

`str.contains('EURO FX')` capture aussi `'EURO FX/JAPANESE YEN XRATE'`,
`'EURO FX/CANADIAN DOLLAR XRATE'`, etc. ; `'JAPANESE YEN'` capture toutes les crosses JPY ;
`'CANADIAN DOLLAR'` capture `'EURO FX/CANADIAN DOLLAR XRATE'`. Combiné à
`sort_values('Date').drop_duplicates('Date')` (B l. 53) qui conserve une ligne
**arbitraire** parmi les marchés partageant la même date (tri quicksort non stable), la
série d'une « devise » peut mélanger plusieurs contrats. Le script A (égalité exacte) est
sûr mais fragile au moindre renommage CFTC (le nom NZD a d'ailleurs varié selon les
époques — d'où la regex du script B). **Correction (§5.2)** : référentiel des marchés +
table d'alias + vérification explicite que chaque marché attendu est trouvé.

### 4.3 Dérive des noms de colonnes

Deux conventions rencontrées selon la version du paquet (§2). Un script figé sur l'une
casse à la mise à jour de l'autre — c'est visiblement ce qui a produit le script B.
**Correction (§5.3)** : couche de résolution (mapping des deux jeux d'en-têtes) + épinglage
des versions.

### 4.4 USD = proxy DXY

`'U.S. DOLLAR INDEX - ICE FUTURES U.S.'` (A l. 30) est le contrat à terme sur l'indice
Dollar Index **pondéré** (euro ~57,6 %). La formation enseigne la lecture de l'**agrégat
USD non pondéré** (`COURSE_METHOD.md` §7). Mesure légitime, mais **différente de celle
enseignée** : elle doit être étiquetée `is_usd_proxy` et affichée « proxy DXY pondéré ».
Jamais confondue avec l'agrégat, et **exclue du classement transversal** (§8, décision
D6 du registre).

### 4.5 Dates incomplètes

Seule la date de référence des positions est lue (`As of Date...` / `Report_Date_as...`).
La **date de publication** (vendredi) et la **date de récupération** ne sont ni extraites
ni conservées. Or la méthode enseignée repose précisément sur le retard mardi → vendredi.

### 4.6 Défauts supplémentaires relevés à l'examen

| # | Défaut | Localisation | Gravité |
|---|---|---|---|
| a | `range(2021, 2027)` codé en dur : cassera en 2027 ; l'année courante partielle peut être silencieusement absente (try/except qui passe) | A l. 46, B l. 24 | moyenne |
| b | OI = 0 → division par zéro → `±inf`, non retiré par `dropna` (qui ne filtre que NaN) | A l. 79-80, B l. 57-59 | moyenne |
| c | Aucune persistance des téléchargements : re-téléchargement complet à chaque exécution, pas de `fetched_at` traçable | global | moyenne |
| d | Artefact final = **PNG seul** : les chiffres ne vivent que dans la console — inexploitable par le journal | A sortie | bloquante pour l'intégration |
| e | `tail(52)` = 52 **observations**, pas 52 semaines calendaires (les trous de jours fériés allongent la fenêtre réelle) | A l. 84, B l. 62 | faible (à documenter) |
| f | Pas de `requirements.txt` épinglé — la dérive de colonnes (§4.3) en est le symptôme direct | global | moyenne |
| g | Aucun test ; aucune détection d'anomalie (semaines manquantes, valeur aberrante) | global | moyenne |
| h | Script B : imports matplotlib/gridspec morts — c'est une variante de debug, pas un outil de production | B l. 3-4 | informative |
| i | `dropna(subset=['Net_Pct_OI'])` (A l. 90, B l. 66) **supprime silencieusement** les lignes dont Long/Short/OI sont manquants dans le fichier source : aucune trace, aucun compte rendu | A l. 90, B l. 66 | moyenne |

## 5. Corrections prévues du moteur (à implémenter en v0.6, après validation)

### 5.1 Percentiles « as-of » (spécification précise)

Pour chaque marché, après tri croissant par `ref_date`, pour chaque observation i
(indices 0-based) :

```
fenêtre(i) = observations [i-51 .. i]      (52 observations, finissant à i incluse ;
                                            si i < 51 : aucune métrique émise)
P20(i) = quantile 0,20 de { Net_Pct_OI[j] ≠ null : j ∈ fenêtre(i) }
P80(i) = quantile 0,80 de { Net_Pct_OI[j] ≠ null : j ∈ fenêtre(i) }
```

- La fenêtre est **expansive puis glissante** : aucune métrique n'est émise avant 52
  observations disponibles → aucun point historique n'utilise de donnée future.
- **Méthode de quantile épinglée** (décision D8) : interpolation **linéaire** sur les
  ordres statistiques — comportement par défaut de `pandas.Series.quantile` /
  `numpy.quantile`. Soit n le nombre de valeurs non nulles de la fenêtre, x₍₀₎ ≤ … ≤ x₍ₙ₋₁₎
  les valeurs triées, α ∈ {0,20 ; 0,80} : h = (n−1)·α, k = ⌊h⌋, puis
  `q = x₍ₖ₎ + (h − k) · (x₍ₖ₊₁₎ − x₍ₖ₎)` (si k+1 > n−1, q = x₍ₙ₋₁₎). Cette définition est
  écrite ici pour qu'une future réimplémentation (Rust ou autre) produise **exactement**
  les mêmes valeurs.
- **Valeurs nulles** : les observations avec `net_pct_oi = null` (OI ≤ 0) restent dans le
  décompte des 52 lignes de la fenêtre mais sont exclues du calcul du quantile (comportement
  pandas, qui ignore les NaN) ; si toutes les valeurs de la fenêtre sont nulles, aucune
  métrique n'est émise.
- Fenêtre comptée **en observations** (pas en semaines calendaires) : choix assumé et
  documenté (défaut e), identique au comportement actuel, simple à vérifier. Le champ du
  contrat se nomme d'ailleurs `window_observations` (décision D7) — l'ancien
  `window_weeks` était un mensonge de dénomination.
- Métriques émises avec leur fenêtre (`window_observations: 52`), leur version de méthode
  (`method_version`, décision D9) et leur date de calcul (`computed_at`) → re-calculs
  futurs traçables.
- Arrondi à 2 décimales à l'export, comme `net_pct_oi`.
- Le PNG conserve ses **lignes droites actuelles** uniquement si l'utilisateur le demande
  explicitement (`--snapshot`), pour compatibilité visuelle avec les anciens dashboards ;
  le défaut devient glissant (légende « 52 dernières observations »).

### 5.2 Référentiel des marchés et alias

Le moteur maintient une correspondance **exacte** (égalité après `strip()`) entre devises
suivies et noms CFTC, alignée sur le référentiel `cot_markets` de la base (migration 0006,
`DATA_MODEL.md`) :

| id | Devise | Nom CFTC principal |
|---|---|---|
| 1 | AUD | `AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE` |
| 2 | NZD | `NEW ZEALAND DOLLAR - CHICAGO MERCANTILE EXCHANGE` |
| 3 | EUR | `EURO FX - CHICAGO MERCANTILE EXCHANGE` |
| 4 | GBP | `BRITISH POUND STERLING - CHICAGO MERCANTILE EXCHANGE` |
| 5 | JPY | `JAPANESE YEN - CHICAGO MERCANTILE EXCHANGE` |
| 6 | CHF | `SWISS FRANC - CHICAGO MERCANTILE EXCHANGE` |
| 7 | CAD | `CANADIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE` |
| 8 | USD | `U.S. DOLLAR INDEX - ICE FUTURES U.S.` (proxy DXY — `is_usd_proxy: true`) |

Règles :

- Le nom NZD a varié selon les époques du fichier CFTC (`NEW ZEALAND DOLLAR - ...` vs
  `NZ DOLLAR - ...`, d'où la regex du script B). Les variantes historiques seront
  ajoutées dans `cot_market_aliases` **après vérification sur le fichier réel** —
  **À VALIDER PAR L'UTILISATEUR** au durcissement (décision D5).
- Un marché attendu introuvable est une **anomalie bloquante à l'export** : le fichier
  JSON n'est pas produit (le moteur affiche l'anomalie `missing_market` et s'arrête).
- Un marché présent dans le fichier mais **inconnu** du référentiel et des alias n'est
  jamais ajouté silencieusement : anomalie `unknown_market`, bloquante.
- Ajouter un alias ou un marché est une action **explicite et versionnée** (migration sur
  le référentiel, ou action dédiée dans Settings plus tard) — jamais un effet de bord
  d'un import.

### 5.3 Résolution des colonnes

Le moteur accepte les deux conventions (officielle CFTC et soulignée) via une table de
correspondance par champ sémantique (`market_name`, `ref_date`, `noncomm_long`,
`noncomm_short`, `open_interest`) et échoue explicitement si un champ est introuvable dans
les deux formes.

### 5.4 Dates

Export systématique de : `ref_date` (mardi, date des positions), `published_at` (vendredi,
si le champ est présent dans le fichier source — sinon `null` et anomalie
`missing_publication_date`), `retrieved_at` (date d'exécution du moteur). Les trois dates
sont conservées indépendamment de bout en bout : `generated_at`/`retrieved_at` dans le
fichier, `imported_at` côté base (décision D2).

### 5.5 Robustesse et reproductibilité

- Plage d'années dynamique : `2019 → année courante` (paramétrable) ; l'échec d'une année
  est journalisé en anomalie `year_unavailable`, pas silencieusement avalé ;
- cache local des fichiers bruts téléchargés (re-exécution sans re-téléchargement) ;
- garde-fou OI ≤ 0 : observation exportée avec `anomalies: ["oi_zero_or_negative"]`,
  `net_pct_oi` à `null` — jamais `±inf` ;
- les lignes dont Long/Short/OI sont manquants ne sont **plus** supprimées silencieusement
  (défaut i) : elles sont comptées et journalisées en `incomplete_row` dans
  `import_anomalies`, puis exclues des observations exportées ;
- `requirements.txt` épinglé (`cot_reports`, `pandas`, `matplotlib`) ;
- tests : échantillon doré (extrait figé du fichier CFTC) → valeurs attendues verrouillées
  pour 3 dates de référence, dont une avec percentile glissant vérifié à la main.

### 5.6 Séparation des semaines manquantes et des marchés absents

- **Marché totalement absent** du fichier produit → `missing_market` : bloquant côté
  moteur (pas de JSON produit) et côté import (fichier rejeté, §7.4).
- **Semaine manquante** dans la série d'un marché présent (écart > 7 jours entre deux
  `ref_date` consécutives) → `missing_market_week` dans `import_anomalies`, avec le nom du
  marché et la date attendue : informatif, non bloquant (trous CFTC légitimes : jours
  fériés, reports). L'UI de fraîcheur (H5) distingue « dernière donnée ancienne » et
  « trou dans la série ».

## 6. Contrat d'échange v1 (JSON, moteur → backend Rust)

Chemin par défaut proposé : le moteur écrit `outputs/cot_observations.json` ; l'utilisateur
l'importe via **Settings → Importer des observations COT** (upload manuel, H1). Le format
est versionné par `format_version` ; le backend refuse les versions inconnues (erreur 422
lisible).

### 6.1 Exemple (tronqué à deux marchés pour lisibilité — un fichier réel contient les huit)

```json
{
  "format_version": "1",
  "generated_at": "2026-11-02T14:35:00Z",
  "generator": {
    "name": "cot-engine",
    "version": "0.2.0",
    "source": "cftc_legacy_futures",
    "retrieved_at": "2026-11-02T14:30:00Z"
  },
  "stats_methods": {
    "net_pct_oi_pctl": {
      "method": "rolling_quantile_linear_interpolation",
      "window_observations": 52,
      "min_observations": 52,
      "version": "1"
    }
  },
  "markets": [
    {
      "market_name": "EURO FX - CHICAGO MERCANTILE EXCHANGE",
      "currency": "EUR",
      "is_usd_proxy": false,
      "proxy_note": null
    },
    {
      "market_name": "U.S. DOLLAR INDEX - ICE FUTURES U.S.",
      "currency": "USD",
      "is_usd_proxy": true,
      "proxy_note": "Contrat ICE Dollar Index (DXY, pondéré) — proxy, pas l'agrégat USD non pondéré enseigné"
    }
  ],
  "observations": [
    {
      "market_name": "EURO FX - CHICAGO MERCANTILE EXCHANGE",
      "currency": "EUR",
      "ref_date": "2026-10-27",
      "published_at": "2026-10-30",
      "long_noncomm": 245123,
      "short_noncomm": 98456,
      "open_interest": 730000,
      "net_noncomm": 146667,
      "net_pct_oi": 20.09,
      "metrics": [
        { "metric": "net_pct_oi_pctl_20", "window_observations": 52, "value": -12.4,
          "method_version": "1", "computed_at": "2026-11-02T14:35:00Z" },
        { "metric": "net_pct_oi_pctl_80", "window_observations": 52, "value": 22.1,
          "method_version": "1", "computed_at": "2026-11-02T14:35:00Z" }
      ],
      "anomalies": []
    },
    {
      "market_name": "NEW ZEALAND DOLLAR - CHICAGO MERCANTILE EXCHANGE",
      "currency": "NZD",
      "ref_date": "2026-10-27",
      "published_at": null,
      "long_noncomm": 32891,
      "short_noncomm": 12304,
      "open_interest": 71234,
      "net_noncomm": 20587,
      "net_pct_oi": 28.9,
      "metrics": [],
      "anomalies": ["missing_publication_date"]
    }
  ],
  "import_anomalies": [
    { "type": "missing_market_week",
      "market_name": "SWISS FRANC - CHICAGO MERCANTILE EXCHANGE", "ref_date": "2026-10-20" }
  ]
}
```

### 6.2 Schéma des champs — objet racine

| Champ | Type | Statut | Règles |
|---|---|---|---|
| `format_version` | string | requis | `"1"` — seule valeur acceptée par le backend v0.6 |
| `generated_at` | horodatage ISO 8601 UTC (`…Z`) | requis | date d'exécution du moteur |
| `generator` | objet | requis | voir §6.3 |
| `stats_methods` | objet | requis | méthodes statistiques déclarées, voir §6.4 |
| `markets` | tableau | requis | **exactement 8 entrées** (les marchés suivis, décision D5) |
| `observations` | tableau | requis | ≥ 1 ; chaque marché suivi doit avoir ≥ 1 observation |
| `import_anomalies` | tableau | requis | vide si le moteur n'a rien détecté |

`generator` :

| Champ | Type | Statut | Règles |
|---|---|---|---|
| `name` | string | requis | ex. `"cot-engine"` |
| `version` | string | requis | semver du moteur (versionné **séparément** du format, décision D9) |
| `source` | string | requis | `"cftc_legacy_futures"` — seule source acceptée en v1 |
| `retrieved_at` | horodatage ISO 8601 UTC | requis | date de téléchargement des archives CFTC |

`stats_methods` : map « id de méthode » → déclaration. En v1, la clé `"net_pct_oi_pctl"`
est requise avec `method`, `window_observations` (52), `min_observations` (52) et
`version` (chaîne). Toute métrique émise doit référencer une méthode déclarée via son
`method_version` = `version` de la méthode.

### 6.3 Schéma des champs — `markets[]`

| Champ | Type | Statut | Règles |
|---|---|---|---|
| `market_name` | string | requis | nom CFTC exact ; doit exister dans `cot_markets` **ou** `cot_market_aliases` (sinon : rejet `unknown_market`) |
| `currency` | string | requis | code ISO (AUD, NZD, EUR, GBP, JPY, CHF, CAD, USD) ; doit exister dans `currencies` et correspondre au référentiel (sinon : rejet `currency_mismatch`) |
| `is_usd_proxy` | booléen | requis | `true` uniquement pour le contrat ICE DXY |
| `proxy_note` | string \| null | requis-nullable | non `null` exigé si `is_usd_proxy` = `true` |

### 6.4 Schéma des champs — `observations[]`

| Champ | Type | Statut | Règles |
|---|---|---|---|
| `market_name` | string | requis | doit être déclaré dans `markets[]` du même fichier |
| `currency` | string | requis | redondance volontaire (robustesse) : doit correspondre au marché, sinon rejet |
| `ref_date` | date `YYYY-MM-DD` | requis | mardi attendu ; autre jour → anomalie `ref_date_not_tuesday` (non bloquante — décalages CFTC exceptionnels) |
| `published_at` | date \| null | requis-nullable | `null` → anomalie `missing_publication_date` ; sinon `ref_date` ≤ `published_at` ≤ `ref_date` + 14 jours (sinon anomalie `publication_date_out_of_range`) |
| `long_noncomm` | entier ≥ 0 | requis | négatif → **rejet du fichier** (`invalid_position`) |
| `short_noncomm` | entier ≥ 0 | requis | idem |
| `open_interest` | entier ≥ 0 | requis | `0` → anomalie `oi_zero_or_negative` + `net_pct_oi` à `null` |
| `net_noncomm` | entier | requis | doit égaler `long_noncomm − short_noncomm` ; écart → anomalie `derived_mismatch`, **le backend stocke sa propre valeur recalculée** (décision D10) |
| `net_pct_oi` | nombre \| null | requis-nullable | `null` ssi `open_interest` ≤ 0 ; sinon `(net_noncomm / open_interest) × 100` arrondi à 2 décimales ; écart > 0,005 → `derived_mismatch`, valeur backend stockée |
| `metrics` | tableau | optionnel | voir §6.5 ; vide si fenêtre incomplète |
| `anomalies` | tableau de codes | requis | vide = rien à signaler ; conservé en base (décision D3) |

`metrics[]` :

| Champ | Type | Statut | Règles |
|---|---|---|---|
| `metric` | string | requis | `net_pct_oi_pctl_20` \| `net_pct_oi_pctl_80` — **liste fermée en v1** |
| `window_observations` | entier | requis | `52` (décision D7 : la fenêtre est comptée en observations) |
| `value` | nombre | requis | arrondi à 2 décimales |
| `method_version` | string | requis | doit exister dans `stats_methods` (sinon rejet `unknown_method`) |
| `computed_at` | horodatage ISO 8601 UTC | requis | date de calcul côté moteur |

`import_anomalies[]` : objets `{ type, …contexte }` — types possibles en v1 :
`missing_market_week` (avec `market_name`, `ref_date`), `year_unavailable` (avec `year`,
`details`), `incomplete_row` (avec `market_name`, `ref_date`, `details`),
`duplicate_observation_collapsed` (avec `market_name`, `ref_date`).

Types de données : « entier » = nombre JSON sans partie fractionnaire ; « horodatage » =
ISO 8601 UTC avec suffixe `Z` ; « date » = `YYYY-MM-DD`. Unités : positions et open
interest en contrats ; `net_pct_oi` et métriques en pourcents.

### 6.5 Identités arithmétiques (vérifiées par le backend, décision D10)

1. `net_noncomm = long_noncomm − short_noncomm` — exact ;
2. `net_pct_oi = net_noncomm / open_interest × 100` (arrondi 2 déc.) — `null` si OI ≤ 0 ;
3. `0 ≤ long_noncomm ≤ open_interest` et `0 ≤ short_noncomm ≤ open_interest` — les positions
   non commerciales sont une sous-catégorie de l'open interest en Legacy Futures ; un
   dépassement est une curiosité de données CFTC : anomalie non bloquante
   (`long_exceeds_oi` / `short_exceeds_oi`), stockée et signalée ;
4. deux observations de même `(market_name, ref_date)` dans un même fichier : **identiques**
   → fusion en une seule + anomalie `duplicate_observation_collapsed` ; **différentes** →
   **rejet du fichier** (`duplicate_conflict`).

**Principe** (audit du 2026-11-03) : le backend ne fait pas confiance aveuglément aux
métriques dérivées envoyées par Python. `net_noncomm` et `net_pct_oi` sont **recalculés**
par le backend à partir des faits (Long/Short/OI) — les valeurs moteur ne servent qu'à la
détection d'écart (`derived_mismatch`). Les percentiles ne sont pas recalculables
intégralement à coût raisonnable : ils sont vérifiés **par échantillonnage** à chaque
import (§7.6). Le classement transversal n'est **pas** transporté par le fichier : calculé
par l'application à la lecture (décision D6).

### 6.6 Versionnement à trois niveaux (décision D9)

| Niveau | Champ | Porte sur | Changement de ce niveau |
|---|---|---|---|
| Format d'échange | `format_version` | la **structure** du JSON (champs, sémantique) | incompatibilité : le backend refuse les versions inconnues |
| Moteur | `generator.version` | le code Python (durcissements, corrections de bugs) | compatible tant que le format et les méthodes ne changent pas |
| Méthodes statistiques | `stats_methods.*.version` + `metrics[].method_version` | la **définition mathématique** des métriques (fenêtre, interpolation) | une évolution de méthode est tracée par métrique, sans bump du format |

## 7. Comportement d'import côté Rust (v0.6)

### 7.1 Réception et détection de ré-import

1. `POST /api/cot/imports` reçoit le fichier JSON (corps de requête). Jamais de
   déclenchement automatique : pas de dossier surveillé, pas de cron, pas d'exécution à
   l'ouverture (H1, T13).
2. Le backend calcule le **SHA-256** du fichier (décision D11). S'il correspond à un import
   antérieur réussi : réponse `409` indiquant l'import précédent (date, id) ; le
   ré-import d'un fichier strictement identique exige une confirmation explicite
   (`force=true`) — et reste de toute façon un no-op idempotent.

### 7.2 Validation intégrale AVANT toute persistance (décision D1)

Le fichier est intégralement parsé et validé **en mémoire** selon §6 (structure, types,
référentiels, identités arithmétiques, doublons internes, complétude des 8 marchés).
Toute erreur bloquante → `422` avec la **liste complète** des problèmes (pas seulement le
premier) et **aucune écriture** en base. Un fichier partiellement invalide n'est jamais
importé partiellement.

### 7.3 Atomicité

Toutes les écritures d'un import (ligne `cot_imports`, observations, métriques) sont
exécutées dans **une transaction SQLite unique** (`BEGIN IMMEDIATE` … `COMMIT`) : l'import
réussit entièrement ou échoue sans laisser de trace partielle. Le compte-rendu
(`inserted` / `updated` / `unchanged`) est mis à jour dans la même transaction.

### 7.4 Référentiels — jamais de création silencieuse (décision D5)

- `markets[]` est confronté à `cot_markets` + `cot_market_aliases` : un marché inconnu des
  deux → rejet `unknown_market`. **Le backend n'insère jamais un marché lui-même.**
- La devise doit exister dans `currencies` et correspondre au référentiel → sinon rejet
  `currency_mismatch`.
- Chacun des **8 marchés suivis** doit être présent dans `markets[]` → sinon rejet
  `missing_market` (un fichier qui « oublie » une devise est un fichier suspects).

### 7.5 Upsert des observations (clé `(market_id, report_type, ref_date)`)

- clé absente → INSERT, `first_import_id` = `last_import_id` = import courant ;
- clé présente, faits identiques (Long/Short/OI/`published_at`/`anomalies`) → **aucun
  changement** (idempotence) ; `first_import_id`/`last_import_id` ne bougent pas — ils
  désignent l'import qui a **écrit** les valeurs courantes, pas celui qui les a confirmées ;
- clé présente, valeurs différentes → UPDATE des faits, `last_import_id` = import courant,
  **et** entrée d'anomalie dans `cot_imports.anomalies` :
  `{type: "value_changed", market, ref_date, champ, ancien, nouveau}` — les données CFTC
  étant normalement immuables, tout écart est une correction ou un changement de méthode :
  il doit être visible (H4).

Les observations présentes en base mais absentes du fichier ne sont **jamais supprimées**
(l'import n'efface rien).

### 7.6 Métriques

- Les `metrics` sont remplacées par observation importée (DELETE + INSERT) : elles sont
  des dérivés recalculés, pas des faits.
- **Vérification par échantillonnage** (décision D10) : pour chaque marché, le backend
  recalcule P20/P80 de la **dernière observation dotée de métriques** à partir des valeurs
  `net_pct_oi` stockées (fenêtre des 52 observations se terminant à cette `ref_date`,
  méthode épinglée §5.1) et compare (tolérance ±0,01). Un écart → anomalie
  `metric_mismatch` journalisée dans `cot_imports.anomalies` (valeur moteur conservée —
  c'est l'autorité statistique ; le contrôle est un détecteur, pas un remplacement).

### 7.7 Journalisation

Un enregistrement `cot_imports` résume chaque import **abouti** : horodatages
(`imported_at` = horloge base, `generated_at`/`retrieved_at` = valeurs du fichier),
générateur (nom, version, source), fichier source + SHA-256, compteurs
(`observations_total`, `inserted`, `updated`, `unchanged`), anomalies agrégées (celles du
fichier + celles détectées à l'import). Les tentatives rejetées ne créent pas de ligne
d'import (rien n'a été persisté) : elles sont visibles dans la réponse `422` et les logs
serveur. Historique des imports consultable (`GET /api/cot/imports`).

## 8. Mesures distinctes — ne jamais les confondre

| Mesure | Définition | Provenance | Calcul | Stockage |
|---|---|---|---|---|
| **Percentile historique glissant (P20/P80)** | bornes de la distribution de `net_pct_oi` d'un marché sur sa fenêtre de 52 observations, calculées « as-of » (sans donnée future) | [OUTIL PERSO], durci [AMÉLIORATION] | moteur Python | `metrics[]` → `cot_metrics` |
| **Position actuelle dans l'histoire propre** | percentile de la dernière valeur dans sa propre fenêtre de 52 observations (même formule de rang, appliquée à l'histoire du marché) | [AMÉLIORATION] | backend, **à la lecture** | non stockée |
| **Classement transversal** | rang du `net_pct_oi` d'un marché parmi les marchés présents à la même `ref_date` ; la formation enseigne la lecture par zones 80–100 | concept : [FORMATION] ; **formule : formalisation Macro Journal [AMÉLIORATION], non vérifiée comme celle du formateur** (décision D6) | backend, **à la lecture** | non stockée |
| **Niveau institutionnel** | données de niveau absolu (CME, « ligne rouge ») | [FORMATION] — externe, jamais reconstruit | — | hors application (raccourci/notes) |
| **Agrégat USD non pondéré** | mesure USD enseignée | [FORMATION] — externe | — | hors application |
| **Proxy DXY** | contrat ICE U.S. Dollar Index (pondéré) | [OUTIL PERSO] | moteur | `is_usd_proxy = 1`, étiquette UI |

**Formule du classement transversal (formalisation nôtre, décision D6)** : pour un marché
m à la date t, `rang = (nombre de marchés non-proxy présents à t avec net_pct_oi
strictement inférieur) / (nombre de marchés présents − 1) × 100`.

- **Égalités** : les marchés ex æquo partagent le même rang (comptage strictement
  inférieur) ;
- **Marchés manquants** : le rang est calculé parmi les marchés **présents** à t et
  affiché avec son décompte (« 5/7 ») pour éviter toute confusion avec un classement sur
  8 ; moins de 2 marchés présents → `null` ;
- **DXY exclu** : le proxy pondéré (euro ~57,6 %) doublerait l'influence de l'EUR et
  comparerait un instrument dérivé à des agrégats de devise ; l'USD agrégé enseigné n'est
  pas disponible dans nos données. La ligne USD reste affichée avec ses percentiles et son
  étiquette « proxy DXY », mais ne participe pas au classement.

L'UI affiche ces mesures avec des libellés distincts et une aide en ligne ; jamais de
fusion sous un « score COT » (anti-exigence #1/#4, H8).

## 9. Ce que le moteur ne fera pas

- Pas d'exécution automatique planifiée ni de lancement à l'ouverture de l'application.
- Pas d'alertes, de scores, de signaux, de recommandations.
- Pas de calcul du classement transversal côté moteur : c'est une lecture applicative sur
  données stockées (décision D6) — le moteur reste concentré sur les statistiques de
  séries temporelles.
- Pas d'extension aux autres rapports CFTC (TFF/disaggregated) sans besoin réel exprimé ;
  le cas échéant, cela se traitera par une évolution **versionnée** du format.
- Pas de réécriture en Rust avant longtemps : le moteur Python est conservé (l'addendum
  l'impose) ; une réécriture ne se justifierait que par un besoin de distribution unique
  — hypothèse explicitement rejetée à ce stade.
- Pas de scraping au-delà de l'API publique du paquet `cot_reports` (téléchargement des
  archives publiques CFTC).

## 10. Emplacement du code [PROPOSITION, à valider]

- **Proposition par défaut** : `tools/cot/` dans le dépôt macro-journal, **hors workspace
  Cargo** (le Python n'y entre jamais), avec `requirements.txt`, `README.md` et les tests ;
  les deux scripts actuels y seraient archivés sous `legacy/` (script A, script B) pour la
  traçabilité, le moteur durci vivant à côté.
- **Alternative** : dépôt/répertoire séparé (`cot-engine/`), si l'utilisateur préfère un
  cycle de vie totalement indépendant de l'application.
- Dans les deux cas : communication **uniquement** par le format d'échange v1 (§6) ; aucun
  import de code Python par le backend, aucun appel de processus par le backend.
