# REQUIREMENTS — Exigences fonctionnelles

> **Traçabilité** : chaque exigence est étiquetée par son origine :
> - **[PROMPT]** exigence explicite du cahier des charges (prompt maître) ;
> - **[FORMATION]** dérive directe de la méthodologie enseignée (voir `COURSE_METHOD.md`) ;
> - **[CARNET]** dérive de l'audit des anciens carnets (voir `OLD_JOURNAL_AUDIT.md`) ;
> - **[DÉDUCTION]** conséquence logique des trois sources ;
> - **[PROPOSITION]** choix de conception proposé, à valider par l'usage.
>
> Les priorités correspondent à la roadmap : **MVP** (v0.2), puis **P2/P3/P4** (v0.3 → v1.0).

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
2. Pas de reconstruction de RateProbability (moteur OIS), d'agrégateur COT complet, de
   clone Trading Economics/Bloomberg/Newsquawk.
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
