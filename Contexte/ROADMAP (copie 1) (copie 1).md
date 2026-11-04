# ROADMAP — Plan de développement de Macro Journal

> Découpage conforme aux phases du cahier des charges (Phase 3 = premier MVP, Phase 4 =
> modules macroéconomiques, Phase 5 = validation). **Une version n'est terminée que lorsque
> sa liste de validation est exécutée et documentée.** Chaque version liste aussi ce qu'elle
> ne fait volontairement PAS.
>
> **Mise à jour du 2026-11-02 [ADDENDUM COT]** : le module COT (moteur Python existant) est
> intégré en **v0.6**, après le MVP et les modules macroéconomiques — sans retarder le
> journal. L'audit des deux scripts et la spécification du format d'échange sont faits
> (`COT_ENGINE.md`) ; l'implémentation attend la validation de cohérence des documents.
>
> **Révision du 2026-11-03** (audit transversal) : spécifications COT corrigées et
> consolidées dans `SPECIFICATION_DECISIONS.md` ; la v0.6 comporte désormais une **porte de
> validation explicite** (étape 0) avant toute ligne de code COT. La place de la v0.6 dans
> la feuille de route et les priorités des autres versions sont **inchangées**.

---

## v0.1.0 — Socle initial (ACCOMPLI)

- Workspace 3 crates (server Axum / web Leptos CSR / shared), SQLite + migration 0001,
  CRUD notes API, éditeur Markdown deux panneaux avec aperçu assaini, interface Dark
  Research, barre latérale repliable (patch).
- Dettes connues documentées dans `ARCHITECTURE.md` §8 (chemins relatifs, pas de tests, pas
  de `Cargo.lock` dans l'archive, monolithes, état sidebar non persistant).

## v0.2.0 — Premier MVP (Phase 3)

**Fonctionnel**
1. Finalisation Dark Research : harmonisation française des chaînes résiduelles, révision
   générale des espacements/hiérarchies, page Settings (emplacement de la base, export,
   sauvegarde, raccourcis externes).
2. Persistance de l'état de la barre latérale (`localStorage`).
3. Navigation complète : les 8 sections du cahier des charges (placeholders honnêtes pour
   les modules non livrés).
4. Research Journal : création/modification/suppression (confirmée), brouillon vs final,
   recherche plein texte (LIKE titre+contenu), filtres (kind, devise, tag, statut).
5. Association notes ↔ devises (référentiel 8 + CNY externe), tags normalisés.
6. Gestion des sources : CRUD, déduplication par URL, liaison aux notes, dates
   publication/consultation.
7. Historique passif : `note_revisions` alimenté à chaque mise à jour (consultation différée
   en v0.5).
8. Stockage : chemin XDG + `MACRO_JOURNAL_DB` + sentinel anti-perte ; chemins statiques
   résolus indépendamment du répertoire courant.
9. Export : dump JSON complet + snapshot SQLite (`VACUUM INTO`) téléchargeable.

**Technique**
- Découpage serveur (`config.rs`, `error.rs`, `routes/`, `services/`) et web (`api.rs`,
  `markdown.rs`, `pages/`, `components/`) — introduit **avec** les fonctionnalités ci-dessus.
- Migration 0002 (voir `DATA_MODEL.md`) + **renommage cohérent `entries` → `notes`** :
  routes `/api/notes`, types partagés `Note`/`NoteDraft`, frontend mis à jour dans la même
  version (décision D12 — atomique avec la migration, un seul vocabulaire).
- `Cargo.lock` versionné après premier build vérifié.

**Validation (à exécuter et consigner)**
- `cargo fmt --all` ; `cargo clippy --workspace` ; `cargo check -p macro-journal-server` ;
  `cargo check -p macro-journal-web --target wasm32-unknown-unknown` ;
- `cargo test --workspace` (tests unitaires validation/normalisation + intégration SQLite
  mémoire : cycle note complète avec révisions, sources dédupliquées, filtres) ;
- scénario manuel : créer → lier devises/tags/sources → modifier (révision créée) →
  rechercher → filtrer → supprimer → exporter → restaurer le snapshot sur une copie ;
- vérifier le comportement anti-perte (base renommée + sentinel présente → erreur claire).

**Hors périmètre v0.2** : événements, narratifs, hebdo, paires, reviews, FTS5,
`leptos_router`, import, intégration COT (v0.6).

## v0.3.0 — Events & News + Currency Research (Phase 4, étapes 1 et 3)

**Fonctionnel**
1. Fiche événement complète (champs chiffrés facultatifs + champs d'opinion séparés,
   catégorie, source, devises multiples) — migration 0003.
2. Liste/filtres événements (devise, catégorie, période) ; lien événement ↔ narratif
   (structure posée, usage pleinement exploité en v0.4).
3. Dossiers Currency Research : les 8 devises + « à surveiller » (CNY) ; agrégation continue
   (dernière synthèse + notes + événements + gabarits d'aide-mémoire par BC : mesure
   d'inflation préférée, spécificités).
4. Gabarits de note optionnels (note quotidienne « avant/après session », note d'analyse à
   trois piliers) — proposés au moment de la création, jamais imposés.
5. Décision `leptos_router` : URLs profondes (`/currencies/USD`, `/journal/42`) — adopté si
   le coût reste mesuré, sinon reporté explicitement.

**Validation** : migration sur copie de la base v0.2 (non destructive vérifiée) ; tests
intégration événements (CRUD + liens n-n) ; scénario manuel de saisie d'une publication
réelle avec triplet réel/consensus/précédent ; vérification de l'agrégation du dossier
devise sans duplication de contenu.

## v0.4.0 — Narrative Tracking + Weekly Outlook (Phase 4, étapes 2 et 4)

**Fonctionnel**
1. Narratifs : création par devise, statuts, **versions datées avec motif de changement**,
   fil chronologique, lien événements (migration 0004).
2. Weekly Outlook : note kind `weekly` + `period_start` (semaine ISO), gabarit aligné sur la
   routine enseignée (événements majeurs, synthèse par devise sur les trois piliers,
   scénarios, **informations manquantes**), reprise par lien des narratifs actifs.
3. Vue « ce qui a changé cette semaine » : nouvelles versions de narratifs + événements
   liés, sur la période.

**Validation** : tests (version_no séquentiel en transaction, non-régression des statuts) ;
scénario complet : créer un narratif → l'alimenter d'un événement → le contester → le
remplacer, puis rejouer l'historique en lecture.

## v0.5.0 — Pair Research + Review + histoire des notes (Phase 4, étapes 5 et 6)

**Fonctionnel**
1. Notes de paire (base/quote) : panneaux reprenant par lien les éléments des deux dossiers
   devise, sections d'analyse relative (divergences, différentiels de politique monétaire,
   facteurs externes, positionnement/pricing, scénario, arguments contraires, invalidation).
2. Review : notes kind `review` structurées (raisonnement / données / interprétation /
   réaction du marché / résultat), liées aux narratifs et événements (migration 0005) ;
   **aucun score, aucun jugement automatique**.
3. Consultation de l'historique des notes (`note_revisions`) : frise par note, lecture
   seule, comparaison visuelle simple de deux versions.

**Validation** : tests des liens n-n note↔event ; scénario « retrouver pourquoi mon analyse
a changé » de bout en bout (recherche → note → révision → événement → version de narratif).

## v0.6.0 — Module COT : intégration du moteur Python existant [ADDENDUM, 2026-11-02]

**Étape 0 — Porte de validation (PRÉALABLE ABSOLU, aucune ligne de code avant elle)** :
1. l'utilisateur **valide le registre des décisions** (`SPECIFICATION_DECISIONS.md`) et les
   points marqués « À VALIDER PAR L'UTILISATEUR » (notamment le nom exact du marché NZD à
   vérifier sur le fichier CFTC réel, l'emplacement `tools/cot/` et le renommage
   `/api/notes` en v0.2) ;
2. le contrat d'échange v1 (`COT_ENGINE.md` §6) et le modèle COT (`DATA_MODEL.md`,
   migration 0006) sont relus conjointement — **la validation du contrat et du modèle de
   données précède l'implémentation COT** (exigence de l'audit du 2026-11-03) ;
3. état des lieux : audit des scripts (fait, `COT_ENGINE.md` §2–4), spécification du
   format v1 (fait, révisé), règles d'import (faites, révisées) — consignés dans
   `DOCUMENTATION_AUDIT_REPORT.md`.

**Fonctionnel**
1. Durcissement du moteur Python (conservé, non réécrit en Rust ; emplacement proposé
   `tools/cot/`, hors workspace Cargo — voir `ARCHITECTURE.md` §4.5) :
   - percentiles **réellement glissants « as-of »** : fenêtre de 52 observations se
     terminant à la date étudiée, méthode d'interpolation épinglée — corrige le biais
     temporel des lignes droites actuelles (calcul au moment du téléchargement, rejoué sur
     tout l'historique) ;
   - correspondance **exacte** des marchés via le référentiel + table d'alias
     (suppression du `str.contains` qui capture les contrats croisés type
     'EURO FX/JAPANESE YEN XRATE') ; vérification du nom NZD sur le fichier réel ;
   - résolution des noms de colonnes selon la version du paquet `cot_reports` (deux
     conventions rencontrées : `Noncommercial Positions-Long (All)` vs
     `NonComm_Positions_Long_All`) ;
   - export des trois dates : référence CFTC (mardi), publication (vendredi),
     récupération ;
   - plage d'années dynamique (2019 → année courante) + cache local des fichiers bruts ;
   - export **JSON (format d'échange v1)** en plus du PNG ; garde-fou OI = 0 (anomalie
     journalisée, `net_pct_oi` à `null`) ; lignes incomplètes journalisées au lieu d'être
     supprimées ; `requirements.txt` épinglé ; tests sur échantillon doré.
2. Backend : migration 0006 révisée (`cot_markets` **sérialisé avec identifiants stables**,
   `cot_market_aliases`, `cot_imports` à trois dates + SHA-256 + compteurs,
   `cot_observations` avec `net_pct_oi` **nullable**, anomalies par ligne et traçabilité
   `first/last_import_id`, `cot_metrics` en fenêtre d'observations + version de méthode —
   voir `DATA_MODEL.md`) ; import **manuel** via `POST /api/cot/imports` : validation
   intégrale avant écriture, **transaction unique**, idempotent, marchés inconnus rejetés,
   dérivés recalculés, corrections journalisées comme anomalies ; endpoints de consultation
   (`markets`, `observations`, `summary` avec fraîcheur et **classement transversal calculé
   à la lecture**, 7 marchés non-proxy).
3. Frontend : bloc « Positionnement COT » dans chaque dossier devise (dernier % OI, P20/P80,
   classement transversal avec décompte « x/7 », fraîcheur avec le retard mardi → vendredi
   expliqué) ; panneau positionnement dans Pair Research (par lien, pas de copie) ; import
   et historique dans Settings ; étiquette « proxy DXY » sur l'USD ; mention permanente
   « indicateur de positionnement, jamais un signal autonome ».

**Validation** : étape 0 franchie et consignée ; ré-importer deux fois le même fichier →
zéro doublon, compteurs `unchanged`, corrections journalisées ; fichier volontairement
corrompu → rejet intégral sans aucune écriture ; vérification manuelle des percentiles
glissants sur 5 dates historiques (contre calcul indépendant) ; test de la vérification par
échantillonnage (`metric_mismatch` sur un fichier trafiqué) ; migration 0006 testée sur
**copie** de la base réelle ; scénario complet moteur → JSON → import → affichage dossier
devise → panneau de paire.

**Hors périmètre v0.6** : téléchargement automatique (jamais — exécution manuelle à la
demande), rapport TFF/disaggregated, alertes ou scores COT, table note↔observation COT (si
un besoin réel apparaît), réécriture du moteur en Rust.

## v1.0.0 — Consolidation

- Revue complète UX Dark Research sur l'ensemble des modules ; responsive petit écran
  (éditeur une colonne, aperçu séparé).
- Documentation utilisateur (README étendu : installation, sauvegarde/restauration,
  philosophie fait/opinion) et documentation technique à jour.
- FTS5 si la recherche LIKE est devenue lente (mesure, pas intuition).
- Import JSON (symétrique de l'export) si le besoin s'est manifesté.
- Préparation « IA-ready » : aucune intégration, seulement la garantie structurelle que les
  suggestions futures resteront distinctes des conclusions (espaces séparés, jamais
  d'écriture silencieuse).

## Règles transverses à chaque version

1. `cargo fmt` + `clippy` + `cargo check` (natif **et** wasm) **exécutés**, pas supposés.
2. Tests verts ; scénarios manuels CRUD/persistance rejoués.
3. Migration testée sur une **copie** de la base réelle ; jamais de destruction.
4. Journal des changements (CHANGELOG.md) tenu ; ce qui n'a pas été testé est écrit noir sur
   blanc.
5. Toute fonctionnalité non justifiée par `REQUIREMENTS.md` est refusée par défaut.

## Points ouvertement reportés (décisions par défaut assumées)

| Sujet | Décision par défaut | Relevable si |
|---|---|---|
| `leptos_router` | introduit en v0.3 (URLs profondes) | coût de migration mesuré |
| FTS5 | LIKE au MVP | lenteur constatée |
| Import | v1.0 | besoin réel exprimé |
| Rappels/calendrier intégré | non : le calendrier économique reste externe | jamais (anti-exigence #2) |
| Déduplication fine des sources (URL normalisée) | exacte après trim au MVP | faux doublons fréquents |
| Téléchargement/exécution automatique du moteur COT | jamais : import manuel à la demande [ADDENDUM] | jamais |
| Rapport CFTC TFF (Traders in Financial Futures) | non : Legacy Futures suffit aux 8 marchés suivis | besoin réel exprimé |
| Liaison note ↔ observation COT | non : affichage via dossiers devise et paires | besoin réel exprimé |
