# Documentation Macro Journal — jeu de spécifications (nov. 2026)

Neuf documents constituent la base documentaire de Macro Journal, mis à jour selon
l'addendum COT du 2026-11-02 puis **révisés par l'audit de cohérence transversal du
2026-11-03** :

| Document | Contenu |
|---|---|
| `COURSE_METHOD.md` | Méthodologie extraite de la formation (durées corrigées : 8 h 58 ; §15 provenance formation / outils perso / améliorations ; §7 : formule du rang transversal = formalisation Macro Journal, jamais attribuée au formateur) |
| `OLD_JOURNAL_AUDIT.md` | Audit des anciens carnets PDF (+ note complémentaire sur l'outil COT externe au carnet) — **inchangé par la révision du 2026-11-03** (aucune erreur factuelle démontrée) |
| `REQUIREMENTS.md` | Exigences fonctionnelles (+ module H « Intégration COT » H1–H13, dont H12 « dérivés non crus aveuglément » et H13 « import atomique et intégral ») |
| `ARCHITECTURE.md` | Architecture cible (+ §4.5 chaîne COT explicite Python → JSON → Rust → SQLite → Leptos ; renommage `entries` → `notes` planifié v0.2) |
| `DATA_MODEL.md` | Modèle de données (diagramme aligné exactement sur les migrations ; migration 0006 COT révisée : 8 marchés sérialisés, alias, `net_pct_oi` nullable, anomalies par ligne, traçabilité imports) |
| `ROADMAP.md` | Plan de versions (MVP v0.2 inchangé ; module COT en v0.6 avec porte de validation étape 0) |
| `COT_ENGINE.md` | Audit intégral des deux scripts Python COT + contrat d'échange JSON v1 complet (schéma champ à champ, identités arithmétiques, versionnement à trois niveaux) + règles d'import Rust (validation intégrale, transaction unique) |
| `SPECIFICATION_DECISIONS.md` | **Nouveau (2026-11-03)** : registre des décisions techniques définitives D1–D14 (import, traçabilité, déduplication, anomalies, référentiel marchés/alias, DXY, rang transversal, percentiles, versionnement, historisation/suppression) + questions ouvertes Q1–Q6 |
| `DOCUMENTATION_AUDIT_REPORT.md` | **Nouveau (2026-11-03)** : rapport de validation — incohérences initiales, corrections réalisées, observations nuancées, décisions à valider, dépendances aux scripts Python, cohérence finale et verdict |

L'archive `macro-journal-docs.zip` contient ces neuf fichiers.
