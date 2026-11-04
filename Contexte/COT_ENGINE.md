# COT_ENGINE — Audit du moteur Python COT et contrat d'échange avec Rust

> **Statut** : établi le 2026-11-02 à la demande de l'addendum COT, **avant toute
> modification de code**. Les deux scripts existants ont été examinés intégralement
> (copies verbatim de référence : `work/cot-engine-reference/script_A_dashboard.py` et
> `script_B_debug_variant.py`).
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
5. Calcul du **positionnement net** (Long − Short).
6. **Normalisation par l'OI** (% OI) — rend les marchés comparables entre eux.
7. Calcul de **percentiles historiques** (P20/P80) — idée correcte, implémentation biaisée
   (§4.1).
8. Génération de graphiques (dashboard 2 colonnes, thème sombre, zone neutre, signal
   visuel sur la dernière valeur, annotations).

## 4. Défauts confirmés (audit intégral)

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
**arbitraire** parmi les marchés partageant la même date, la série d'une « devise » peut
mélanger plusieurs contrats. Le script A (égalité exacte) est sûr mais fragile au moindre
renommage CFTC (le nom NZD a d'ailleurs varié selon les époques — d'où la regex du script
B). **Correction (§5.2)** : table de correspondance exacte documentée + vérification
explicite que chaque marché attendu est trouvé.

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
Jamais confondue avec l'agrégat.

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

## 5. Corrections prévues du moteur (à implémenter en v0.6, après validation)

### 5.1 Percentiles « as-of » (spécification précise)

Pour chaque marché, après tri croissant par `ref_date`, pour chaque observation i :

```
P20(i) = quantile 0,20 de Net_Pct_OI[i-51 .. i]   (fenêtre de 52 observations,
P80(i) = quantile 0,80 de Net_Pct_OI[i-51 .. i]    finissant à i incluse ;
                                                    si i < 51 : pas de métrique
                                                    émise — window incomplete)
```

- La fenêtre est **expansive puis glissante** : aucune métrique n'est émise avant 52
  observations disponibles → aucun point historique n'utilise de donnée future.
- Fenêtre comptée **en observations** (pas en semaines calendaires) : choix assumé et
  documenté (e), identique au comportement actuel, simple à vérifier.
- Métriques émises avec leur fenêtre (`window_weeks: 52`) et leur date de calcul
  (`computed_at`) → re-calculs futurs traçables.
- Le PNG conserve ses **lignes droites actuelles** uniquement si l'utilisateur le demande
  explicitement (`--snapshot`), pour compatibilité visuelle avec les anciens dashboards ;
  le défaut devient glissant.

### 5.2 Correspondance exacte des marchés

Table de correspondance **exacte** (égalité après `strip()`), maintenue dans le moteur :

| Devise | Nom CFTC attendu (à vérifier sur le fichier réel au durcissement) |
|---|---|
| AUD | `AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE` |
| NZD | `NEW ZEALAND DOLLAR - CHICAGO MERCANTILE EXCHANGE` **ou** `NZ DOLLAR - ...` (nom ayant varié — à trancher sur données réelles) |
| EUR | `EURO FX - CHICAGO MERCANTILE EXCHANGE` |
| GBP | `BRITISH POUND STERLING - CHICAGO MERCANTILE EXCHANGE` |
| JPY | `JAPANESE YEN - CHICAGO MERCANTILE EXCHANGE` |
| CHF | `SWISS FRANC - CHICAGO MERCANTILE EXCHANGE` |
| CAD | `CANADIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE` |
| USD | `U.S. DOLLAR INDEX - ICE FUTURES U.S.` (proxy DXY — `is_usd_proxy: true`) |

Un marché attendu introuvable est une **anomalie bloquante à l'export** (le fichier JSON
n'est pas produit), pas un avertissement en console. Un alias historique peut être ajouté à
la table après vérification documentée.

### 5.3 Résolution des colonnes

Le moteur accepte les deux conventions (officielle CFTC et soulignée) via une table de
correspondance par champ sémantique (`market_name`, `ref_date`, `noncomm_long`,
`noncomm_short`, `open_interest`) et échoue explicitement si un champ est introuvable dans
les deux formes.

### 5.4 Dates

Export systématique de : `ref_date` (mardi, date des positions), `published_at` (vendredi,
si le champ est présent dans le fichier source — sinon `null` et anomalie `missing_publication_date`),
`retrieved_at` (date d'exécution du moteur).

### 5.5 Robustesse et reproductibilité

- Plage d'années dynamique : `2019 → année courante` (paramétrable) ; l'échec d'une année
  est journalisé en anomalie, pas silencieusement avalé ;
- cache local des fichiers bruts téléchargés (re-exécution sans re-téléchargement) ;
- garde-fou OI ≤ 0 : observation exportée avec `anomalies: ["oi_zero_or_negative"]`,
  `net_pct_oi` à `null` — jamais `±inf` ;
- `requirements.txt` épinglé (`cot_reports`, `pandas`, `matplotlib`) ;
- tests : échantillon doré (extrait figé du fichier CFTC) → valeurs attendues verrouillées
  pour 3 dates de référence, dont une avec percentile glissant vérifié à la main.

## 6. Contrat d'échange v1 (JSON, moteur → backend Rust)

Chemin par défaut proposé : le moteur écrit `outputs/cot_observations.json` ; l'utilisateur
l'importe via **Settings → Importer des observations COT** (upload manuel, H1). Le format
est versionné par `format_version` ; le backend refuse les versions inconnues.

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
        { "metric": "net_pct_oi_pctl_20", "window_weeks": 52, "value": -12.4, "computed_at": "2026-11-02T14:35:00Z" },
        { "metric": "net_pct_oi_pctl_80", "window_weeks": 52, "value": 22.1, "computed_at": "2026-11-02T14:35:00Z" },
        { "metric": "cross_sectional_rank", "window_weeks": null, "value": 88.0, "computed_at": "2026-11-02T14:35:00Z" }
      ],
      "anomalies": []
    }
  ],
  "import_anomalies": [
    { "type": "missing_market_week", "market_name": "SWISS FRANC - CHICAGO MERCANTILE EXCHANGE", "ref_date": "2026-10-20" }
  ]
}
```

**Champs — correspondance avec l'exigence H2 :**

| Exigence H2 | Champ(s) |
|---|---|
| Marché / instrument | `markets[].market_name` |
| Devise représentée | `markets[].currency` + `observations[].currency` |
| Date de référence CFTC | `observations[].ref_date` (mardi) |
| Date de publication | `observations[].published_at` (vendredi) |
| Date de récupération | `generator.retrieved_at` (+ `cot_imports.imported_at` côté base) |
| Positions Long / Short | `long_noncomm` / `short_noncomm` |
| Open Interest | `open_interest` |
| Net Positioning | `net_noncomm` |
| Net Positioning % OI | `net_pct_oi` (null si OI ≤ 0) |
| Percentiles historiques | `metrics[]` (`net_pct_oi_pctl_20/80`, `window_weeks`) |
| Fraîcheur | dernière `ref_date` par marché + `published_at` + `retrieved_at`/`imported_at` |
| Anomalies | `observations[].anomalies` (par ligne) + `import_anomalies` (par fichier) |

**Définitions verrouillées :**

- `net_pct_oi` : `(long_noncomm - short_noncomm) / open_interest × 100`, arrondi à 2
  décimales à l'export (la base stocke la valeur exportée, pas un recalcul) ;
- `cross_sectional_rank` (rang transversal enseigné, à ne pas confondre avec les
  percentiles) : rang du `net_pct_oi` du marché parmi les marchés disposant d'une
  observation à la **même `ref_date`**, exprimé en 0–100 par
  `(nombre de marchés strictement inférieurs) / (nombre de marchés − 1) × 100` ;
  si un seul marché présent → `null` ;
- toutes les dates en ISO 8601 ; heures en UTC avec suffixe `Z` pour les horodatages.

## 7. Comportement d'import côté Rust (v0.6)

1. `POST /api/cot/imports` reçoit le fichier JSON ; validation du `format_version`
   (refus explicite si inconnu — erreur 422 lisible).
2. Chaque observation est upsertée sur la clé `(market_id, report_type, ref_date)` :
   - clé absente → INSERT ;
   - clé présente, valeurs identiques → aucun changement (idempotence) ;
   - clé présente, valeurs différentes → UPDATE **et** entrée d'anomalie dans
     `cot_imports.anomalies` : `{market, ref_date, champ, ancien, nouveau}` — les données
     CFTC étant normalement immuables, tout écart est une correction ou un changement de
     méthode : il doit être visible.
3. Les `metrics` sont remplacées par observation importée (DELETE + INSERT) : elles sont
   des dérivés recalculés, pas des faits.
4. Les `markets` sont synchronisés avec `cot_markets` (référentiel) ; un marché inconnu de
   la base est inséré ; une devise inconnue est refusée (le référentiel `currencies` fait
   foi).
5. Un enregistrement `cot_imports` résume l'import : horodatage, générateur, fichier
   source, nombre d'observations, anomalies agrégées.
6. Jamais de déclenchement automatique : pas de dossier surveillé, pas de cron, pas
   d'exécution à l'ouverture (H1, T13).

## 8. Mesures distinctes — ne jamais les confondre

| Mesure | Définition | Provenance | Où |
|---|---|---|---|
| **Rang transversal enseigné** | Classement des devises entre elles à une date donnée ; « zones 80–100 = suracheté » | [FORMATION] (`COURSE_METHOD.md` §7) | `cross_sectional_rank` |
| **Percentile historique glissant** | Position d'une valeur dans sa **propre** série sur 52 semaines (P20/P80), calculé « as-of » | [OUTIL PERSO] / [AMÉLIORATION] | `net_pct_oi_pctl_20/80` |
| **Niveau institutionnel** | Données de niveau absolu (CME, « ligne rouge ») | [FORMATION] — externe, jamais reconstruit | hors application (raccourci/notes) |
| **Agrégat USD non pondéré** | mesure USD enseignée | [FORMATION] — externe | hors application |
| **Proxy DXY** | Contrat ICE U.S. Dollar Index (pondéré) | [OUTIL PERSO] | `is_usd_proxy = 1`, étiquette UI |

L'UI affiche ces mesures avec des libellés distincts et une aide en ligne ; jamais de
fusion sous un « score COT » (anti-exigence #1/#4, H8).

## 9. Ce que le moteur ne fera pas

- Pas d'exécution automatique planifiée ni de lancement à l'ouverture de l'application.
- Pas d'alertes, de scores, de signaux, de recommandations.
- Pas d'extension aux autres rapport CFTC (TFF/disaggregated) sans besoin réel exprimé.
- Pas de réécriture en Rust avant longtemps : le moteur Python est conservé (l'addendum
  l'impose) ; une réécriture ne se justifierait que par un besoin de distribution unique
- hypothèse explicitement rejetée à ce stade.
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
