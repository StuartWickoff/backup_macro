# REQUIREMENTS — Exigences fonctionnelles

> **Traçabilité** : chaque exigence est étiquetée par son origine :
> - **[PROMPT]** exigence explicite du cahier des charges (prompt maître) ;
> - **[FORMATION]** dérive directe de la méthodologie enseignée (voir `COURSE_METHOD.md`) ;
> - **[CARNET]** dérive de l'audit des anciens carnets (voir `OLD_JOURNAL_AUDIT.md`) ;
> - **[DÉDUCTION]** conséquence logique des trois sources ;
> - **[PROPOSITION]** choix de conception proposé, à valider par l'usage ;
> - **[ADDENDUM]** exigence ou correction posée par l'addendum COT (nov. 2026), qui prévaut
>   sur les passages contradictoires des documents existants ; les outils personnels de
>   l'utilisateur y sont étiquetés **[OUTIL PERSO]** quand la distinction de provenance
>   importe (voir `COURSE_METHOD.md` §15) ;
> - **[RÉVISION]** exigence ajoutée ou précisée par l'audit de cohérence documentaire du
>   2026-11-03 (voir `DOCUMENTATION_AUDIT_REPORT.md` et `SPECIFICATION_DECISIONS.md`).
>
> Les priorités correspondent à la roadmap : **MVP** (v0.2), puis **P2/P3/P4** (v0.3 → v0.5),
> **v0.6** pour le module H (intégration COT), **v1.0** pour la consolidation.

---

## 1. Exigences transversales

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| T1 | Application 100 % locale, accessible sur `127.0.0.1`, sans cloud, sans compte, sans clé API | [PROMPT] | MVP |
| T2 | Séparation stricte fait / interprétation / implication monétaire / pricing / conséquence devises, au niveau des champs et des gabarits | [PROMPT][FORMATION] | MVP (gabarits) |
| T3 | L'incertitude est un état de première classe : biais haussier / baissier / neutre / indéterminé ; arguments contradictoires ; narratif en construction ; informations insuffisantes | [PROMPT][FORMATION] | MVP |
| T4 | Historique : aucune mise à jour n'écrase silencieusement l'ancienne version ; toute note conserve ses révisions | [PROMPT] | MVP (écriture) / P4 (consultation) |
| T5 | Provenance : toute information importante peut être reliée à une source (nom, URL, date de publication, date de consultation, type) | [PROMPT] | MVP |
| T6 | Base SQLite dans un emplacement stable XDG (`~/.local/share/macro-journal/`), configurable ; refus de création silencieuse d'une base vide quand une base existante est attendue | [PROMPT][DÉDUCTION] | MVP |
| T7 | Migrations SQL versionnées, non destructives | [PROMPT] | MVP |
| T8 | Export/sauvegarde simple et vérifiable (dump JSON de toutes les tables + copie du fichier SQLite) | [PROMPT] | MVP |
| T9 | Interface Dark Research : sombre, sobre, typographie lisible, couleurs réservées à l'information, animations discrètes, pas d'esthétique CRUD générique | [PROMPT] | MVP (consolidation) |
| T10 | Barre latérale repliable, état conservé entre les sessions | [PROMPT] | MVP (persistance) |
| T11 | Navigation : Overview, Research Journal, Weekly Outlook, Currencies, Events & News, Pair Research, Reviews, Settings — sans sous-menus superflus | [PROMPT] | MVP (structure) |
| T12 | Priorité desktop Linux/Firefox ; dégradation propre sur petit écran (éditeur une colonne) | [PROMPT] | MVP |
| T13 | Ressources modestes : pas de dépendance lourde, pas de traitement en arrière-plan | [PROMPT] | MVP |
| T14 | L'application fonctionne intégralement sans IA ; aucun abonnement indispensable | [PROMPT] | permanent |
| T15 | Recherche plein texte (titre + contenu) et filtres (type, devise, tag, période) | [PROMPT][CARNET] | MVP |
| T16 | Raccourcis vers les services externes (RateProbability, CFTC, calendriers, ING, Newsquawk…) et enregistrement de liens les rattachant aux analyses | [PROMPT][FORMATION] | MVP (liens) |

## 2. Module A — Research Journal

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| A1 | Créer / modifier / supprimer une note (confirmation avant suppression) | [PROMPT] | MVP |
| A2 | Enregistrer un brouillon (statut brouillon vs final) | [PROMPT] | MVP |
| A3 | Éditeur Markdown **deux panneaux** conservé, aperçu assaini (ammonia) correctement stylisé | [PROMPT] | MVP (existant) |
| A4 | Associer une note à une ou plusieurs devises (dont la Chine en rôle externe) | [PROMPT][CARNET] | MVP |
| A5 | Tags libres, normalisés (minuscules, uniques) | [PROMPT] | MVP |
| A6 | Lier la note à des sources existantes ou nouvelles | [PROMPT] | MVP |
| A7 | Lister les notes récentes, triées par mise à jour | [PROMPT] | MVP (existant) |
| A8 | Rechercher et filtrer (T15) | [PROMPT] | MVP |
| A9 | Gabarits de note optionnels (note quotidienne « avant/après session », note d'analyse à trois piliers, note d'événement) — proposés, jamais imposés | [FORMATION][PROPOSITION] | P2 |
| A10 | Rédaction fluide : aucun champ obligatoire hormis le titre ; les métadonnées restent optionnelles | [PROMPT] | MVP |

## 3. Module B — Currency Research

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| B1 | Huit dossiers permanents : USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD (+ Chine en rôle externe) | [PROMPT][CARNET] | P2 |
| B2 | Le dossier agrège **en continu** : notes liées, événements liés, narratifs actifs, dernière synthèse — remplace l'instantané mensuel | [PROMPT][CARNET][DÉDUCTION] | P2 |
| B3 | Vue « situation actuelle » = dernière note de synthèse de la devise (avec son historique de révisions), pas une fiche statique | [PROMPT][DÉDUCTION] | P2 |
| B4 | Référentiel par devise : banque centrale, mesure d'inflation préférée, spécificités (3 taux BCE, trimestriel NZ, measures core CAD…) — affiché en aide-mémoire documenté, éditable | [FORMATION][PROPOSITION] | P2 |
| B5 | Aucune donnée de marché fabriquée : si une information manque, la section reste vide et datée | [PROMPT] | permanent |

## 4. Module C — Weekly Outlook

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| C1 | Une analyse hebdomadaire datée (semaine ISO), liée aux devises importantes de la semaine | [PROMPT][FORMATION] | P3 |
| C2 | Structure de la routine du week-end : événements majeurs attendus, publications clés (calendrier), synthèse par devise sur les trois piliers, scénarios envisagés | [FORMATION] | P3 |
| C3 | Section « informations encore manquantes » explicite | [PROMPT] | P3 |
| C4 | Reprise des narratifs actifs des devises concernées (liens, pas copie) | [PROMPT][DÉDUCTION] | P3 |
| C5 | Comparaison facilitée semaine passée / semaine courante | [DÉDUCTION] | P4 |

## 5. Module D — Events & News

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| D1 | Fiche événement : titre, date/heure, source (URL), devises concernées (n), catégorie | [PROMPT] | P2 |
| D2 | Champs chiffrés facultatifs : réel, consensus, précédent, précédent révisé, unité/mesure | [PROMPT][CARNET] | P2 |
| D3 | Champs narratifs séparés des chiffrés : résumé factuel / interprétation / conséquence macro possible / conséquence sur les anticipations monétaires / réaction du marché | [PROMPT][FORMATION] | P2 |
| D4 | Liaison événement ↔ narratifs concernés | [PROMPT] | P2 (structure) / P3 (usage) |
| D5 | Catégories alignées sur la taxinomie de la formation : taux, inflation, emploi, croissance, PMI, discours, géopolitique, positionnement, autre | [FORMATION] | P2 |
| D6 | Un même événement concerne plusieurs devises et alimente plusieurs analyses — relation, pas copie | [PROMPT] | P2 |
| D7 | Filtrer par devise, catégorie, période | [DÉDUCTION] | P2 |

## 6. Module E — Narrative Tracking

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| E1 | Un narratif = titre + devise + statut (en construction / actif / contesté / remplacé / archivé) | [PROMPT][DÉDUCTION] | P3 |
| E2 | Versions datées : chaque évolution significative ajoute une version (arguments, données disponibles, conséquence monétaire envisagée, ce qui a changé et pourquoi) | [PROMPT] | P3 |
| E3 | Le motif de changement est explicite (chiffre, discours, géopolitique…) et peut référencer les événements liés | [PROMPT][FORMATION] | P3 |
| E4 | Fil chronologique lisible (v1 → vn) ; consultation d'une version ancienne sans la modifier | [PROMPT] | P3 |
| E5 | Simplicité volontaire : pas de diff automatique, pas de branches ; une liste de versions datées suffit | [PROMPT][PROPOSITION] | P3 |

## 7. Module F — Pair Research

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| F1 | Une note de paire = deux devises ordonnées (base/quote), sans dupliquer les dossiers devise | [PROMPT] | P4 |
| F2 | Panneaux côte à côte reprenant **les derniers éléments des deux dossiers** (notes, événements, narratifs actifs) via des liens | [PROMPT][DÉDUCTION] | P4 |
| F3 | Sections d'analyse relative : divergences fondamentales, différentiels de politique monétaire, facteurs externes, positionnement/pricing, événements à venir, scénario relatif, arguments contraires, conditions d'invalidation | [PROMPT][FORMATION] | P4 |
| F4 | Gabarit inspiré de la logique « differentials » de la formation (comparer, ne pas répéter) | [FORMATION] | P4 |

## 8. Module G — Review

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| G1 | Comparer les hypothèses passées (notes hebdo, narratifs) avec les événements et évolutions observés ensuite | [PROMPT][FORMATION] | P4 |
| G2 | Distinction explicite : qualité du raisonnement / qualité des données / exactitude de l'interprétation / réaction effective du marché / résultat éventuel d'une opération | [PROMPT] | P4 |
| G3 | Principle de non-jugement par le résultat : une analyse pertinente peut précéder un mouvement contraire à court terme — l'UI ne doit pas afficher de « score » | [PROMPT] | permanent |
| G4 | Le review se lie aux narratifs et événements concernés | [DÉDUCTION] | P4 |

## 9. Exigences non fonctionnelles (extraits)

- Code : Rust idiomatique, types explicites, erreurs traitées, modèles/services/handlers
  séparés, pas de logique métier dans les composants, commentaires sur le non-évident,
  lisibilité pour un apprenant Rust (chapitres 1–8 du Rust Book) [PROMPT].
- Validation par version : `cargo fmt`, `cargo check` (natif + wasm32), `clippy`, tests
  disponibles, scénarios CRUD + persistance vérifiés, rapport honnête de ce qui a été testé
  [PROMPT].
- Compilation reproductible : `Cargo.lock` versionné [DÉDUCTION — absent de l'archive v0.1.0].
- L'IA (plus tard) : suggestions distinctes des conclusions, jamais de modification
  silencieuse [PROMPT].

## 10. Anti-exigences (ce que Macro Journal ne fera PAS)

1. Pas de dashboard à graphiques décoratifs, pas de KPI multiples, pas de scores par devise.
2. Pas de reconstruction de RateProbability (moteur OIS), de clone Trading
   Economics/Bloomberg/Newsquawk, ni de plateforme COT généraliste (multi-marchés, alertes,
   réplique de l'historique CFTC complet). **Correction [ADDENDUM, nov. 2026]** : le
   traitement COT des 8 marchés suivis est notre moteur Python existant — téléchargement,
   normalisation, calcul — conservé et durci, ses résultats étant importés **manuellement**
   dans l'application (module H, §12 ; spécification dans `COT_ENGINE.md`). Le CFTC reste le
   fournisseur externe des rapports bruts ; RateProbability reste externe (raccourci +
   note d'observation datée, sans intégration API).
3. Pas de scraping de sites protégés ou authentifiés ; pas d'API externe sans justification
   fonctionnelle.
4. Pas de système de trading, pas de signaux automatiques, pas d'addition de points.
5. Pas de données actuelles fictives, pas de fausses analyses de démonstration pré-remplies.
6. Pas de conversion des carnets PDF en application de tableaux statiques.
7. Pas de dépendance à une IA, un abonnement ou une clé API.
8. Pas de réécriture complète du projet quand une évolution ciblée suffit.

## 11. Risques identifiés et parades

| Risque | Parade |
|---|---|
| Transformer chaque notion de la formation en champ de formulaire | gabarits **optionnels** ; le contenu reste du Markdown libre ; champs structurés uniquement pour les événements (là où l'ancien carnet montrait des erreurs de saisie) |
| Duplication du contenu entre dossiers devise, paires et hebdo | tout est relation (note_currencies, event_narratives…) ; les vues agrègent par lien, jamais par copie |
| Complexité prématurée (versioning lourd, traits génériques) | narratives = 2 tables ; révisions = 1 table d'archives ; pas d'abstraction avant le 2ᵉ besoin réel |
| Écrasement silencieux de l'historique | `note_revisions` alimentée à chaque mise à jour dès le MVP |
| Base perdue / recréée vide | chemin XDG stable + sentinel `.initialized` + message d'erreur explicite si base attendue introuvable |
| Enlisement dans le design avant le socle | la roadmap fige le MVP sur des fonctionnalités, pas sur de la cosmétique |
| Réintroduire le biais temporel des percentiles COT (calculés « à la date d'aujourd'hui » puis rejoués sur tout l'historique) | définition « as-of » imposée au moteur Python (`COT_ENGINE.md` §5.1) + vérification manuelle sur 5 dates historiques |
| Lire le COT comme un signal (alertes, scores, recommandations) | H8 permanent ; distinction affichée entre percentile historique, position actuelle et classement transversal (concept enseigné — formule : formalisation Macro Journal, jamais attribuée au formateur) |
| Lire la ligne USD (contrat ICE DXY pondéré) comme l'agrégat USD non pondéré enseigné | étiquette « proxy DXY » dans l'UI + documentation dédiée (H7, `COT_ENGINE.md` §8) ; **exclusion du proxy du classement transversal** avec décompte affiché (« x/7 ») pour éviter la double pondération de l'EUR |
| Importer des données COT incohérentes ou corrompues sans s'en apercevoir | validation intégrale avant persistance (rejet = zéro écriture), transaction unique, dérivés recalculés côté backend, percentiles vérifiés par échantillonnage, anomalies conservées et affichées (H12, `COT_ENGINE.md` §6-7) |

## 12. Module H — Intégration COT (ajout [ADDENDUM], nov. 2026)

> Section ajoutée en fin de document pour ne pas renuméroter les références existantes
> (l'addendum visait « REQUIREMENTS.md, section 10 » pour l'anti-exigence corrigée
> ci-dessus). Priorité : **v0.6**, sauf mention contraire. Contexte et spécification
> technique complète : `COT_ENGINE.md`. **Le moteur Python COT est un outil personnel
> EXISTANT de l'utilisateur [OUTIL PERSO]** : il est intégré tel quel (durci, jamais
> réécrit) — ce n'est pas une fonctionnalité à construire from scratch.

| ID | Exigence | Origine | Priorité |
|---|---|---|---|
| H1 | Import **manuel, à la demande** des observations COT normalisées (fichier JSON au format d'échange documenté et versionné) ; jamais de téléchargement automatique à l'ouverture de l'application | [ADDENDUM] | v0.6 |
| H2 | Le format d'échange préserve : marché/instrument, devise représentée, date de référence CFTC (mardi), date de publication (vendredi), date de récupération, positions Long/Short non commerciales, Open Interest, Net Positioning, Net % OI, percentiles historiques, fraîcheur et anomalies | [ADDENDUM] | v0.6 |
| H3 | Les observations COT sont stockées comme **données structurées** (jamais comme notes Markdown), uniques par (marché, type de rapport, date de référence) ; l'import est idempotent et non destructif | [ADDENDUM] | v0.6 |
| H4 | Un ré-import qui modifie une valeur existante est appliqué **et journalisé comme anomalie** (les données CFTC publiées étant normalement immuables) — jamais de changement silencieux | [ADDENDUM][DÉDUCTION] | v0.6 |
| H5 | Le dossier de chaque devise affiche la dernière observation, ses métriques et sa fraîcheur (retard mardi → vendredi expliqué dans l'aide en ligne) | [ADDENDUM] | v0.6 |
| H6 | Distinction **documentée et visible** entre le percentile historique glissant (P20/P80 sur une fenêtre de 52 **observations** ≈ 52 semaines, notre implémentation), la position actuelle dans l'histoire propre du marché et le classement transversal enseigné (zones 80–100 des devises entre elles — la **formule précise** du rang est une formalisation Macro Journal calculée par l'application, jamais attribuée au formateur) | [ADDENDUM][FORMATION][RÉVISION] | v0.6 |
| H7 | L'USD est représenté par le contrat ICE U.S. Dollar Index : **proxy DXY pondéré**, signalé comme tel à l'affichage, distinct de l'agrégat USD non pondéré enseigné | [ADDENDUM][FORMATION] | v0.6 |
| H8 | Le COT reste un indicateur de positionnement, **jamais un signal autonome** : pas d'alerte, pas de score, pas de recommandation générés à partir des observations | [ADDENDUM][FORMATION] | permanent |
| H9 | Pair Research reprend les observations COT des deux devises **par lien** (pas de copie) | [ADDENDUM][DÉDUCTION] | v0.6 |
| H10 | Le moteur Python reste **indépendant** de l'application (pas de dépendance WASM, pas d'orchestration par le backend) ; la communication se fait uniquement par fichiers au format documenté | [ADDENDUM] | permanent |
| H11 | Toute évolution du schéma COT est versionnée (migration) et testée sur une copie de la base existante avant application | [ADDENDUM] | permanent |
| H12 | Le backend **ne fait pas confiance aveuglément aux métriques dérivées** envoyées par le moteur : `net_noncomm` et `net_pct_oi` sont recalculés à partir des faits (Long/Short/OI) — tout écart est journalisé (`derived_mismatch`) ; les percentiles sont vérifiés par échantillonnage à chaque import (`metric_mismatch`) ; le classement transversal n'est pas transporté par le fichier (calcul applicatif à la lecture) | [RÉVISION] | v0.6 |
| H13 | L'import COT est **atomique et intégral** : validation complète du fichier avant toute écriture (rejet = zéro modification de la base, erreur détaillée), une seule transaction SQLite, journalisation de chaque import abouti (compteurs insérées/mises à jour/inchangées, SHA-256 du fichier) ; un marché inconnu du référentiel et de ses alias est **rejeté**, jamais créé silencieusement | [RÉVISION] | v0.6 |
