# ROADMAP — Plan de développement de Macro Journal

> Découpage conforme aux phases du cahier des charges (Phase 3 = premier MVP, Phase 4 =
> modules macroéconomiques, Phase 5 = validation). **Une version n'est terminée que lorsque
> sa liste de validation est exécutée et documentée.** Chaque version liste aussi ce qu'elle
> ne fait volontairement PAS.

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
- Migration 0002 (voir `DATA_MODEL.md`).
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
`leptos_router`, import.

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

## Points ouvertment reportés (décisions par défaut assumées)

| Sujet | Décision par défaut | Relevable si |
|---|---|---|
| `leptos_router` | introduit en v0.3 (URLs profondes) | coût de migration mesuré |
| FTS5 | LIKE au MVP | lenteur constatée |
| Import | v1.0 | besoin réel exprimé |
| Rappels/calendrier intégré | non : le calendrier économique reste externe | jamais (anti-exigence #2) |
| Déduplication fine des sources (URL normalisée) | exacte au MVP | faux doublons fréquents |
