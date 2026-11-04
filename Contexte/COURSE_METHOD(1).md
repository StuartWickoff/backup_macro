# COURSE_METHOD — Méthodologie extraite de la formation

> **Statut** : document de référence établi en Phase 0/1 (audit intégral du corpus).
> **Source** : 74 vidéos (42 module de base + 32 module avancé), transcriptions Whisper medium
> (TXT + SRT), **8 h 58** de contenu (5 h 29 base + 3 h 29 avancé), examinées intégralement.
> *Correction du 2026-11-02 [ADDENDUM]* : une version antérieure de ce document annonçait
> « ~13 h 40 » ; le recalcul depuis `audit_durees.csv` (somme des durées vidéo réelles)
> donne **8 h 58 min**. Les durées par sous-module du §1 ont été corrigées dans la même
> passe.
> **Conventions** : [PROFOND] = enseigné en profondeur ; [ÉVOQUÉ] = simplement mentionné ;
> [À VÉRIFIER] = affirmation du formateur économiquement discutable, datée, ou déformée par la
> transcription — à recouper avec une source externe avant d'être tenue pour vraie ;
> [OUTIL PERSO] = outil ou implémentation développée indépendamment par l'utilisateur, hors
> formation (voir §15) ; [AMÉLIORATION] = choix de méthode personnel, postérieur à la
> formation, assumé comme tel.
> **Règle de fidélité** : ce document décrit ce que dit la formation. Il ne corrige pas
> silencieusement le formateur et n'invente aucune méthodologie absente du corpus. Il
> n'attribue pas non plus à la formation les outils personnels de l'utilisateur.

---

## 1. Structure de la formation (noms et organisation originaux conservés)

### Module de base (42 vidéos)

| Sous-module | Vidéos | Durée cumulée |
|---|---|---|
| `introduction` | Mais c'est quoi l'analyse fondamentale · La nature fondamentale du forex · Exemples concrets | ~24 min |
| `elements_macro` | C'est quoi la macro · Taux d'intérêts · Inflation · Emploi · Pib · Étude de cas BOC octobre 2024 | ~73 min |
| `banques_centrales` | Banques centrales et leurs rôles · Politiques monétaires · Comment les banques centrales impactent le forex · Cas concret USD | ~39 min |
| `banques_centrales_majeures` | fed · ecb · boe · boj · snb · boc · rba · rbnz | ~48 min |
| `calendrier_eco` | Généralités · Annonces majeures · Annonce secondaire · Annonces tertiaires · Comment interpréter une annonce économique _ Cas pratiques · Comment sont trouvées les prévisions du marché | ~53 min |
| `les_differents_types_de_devise` | Introduction · Policy driven · Commodity · Devises refuges (safe haven) · Devises d'exportations | ~17 min |
| `correlations` | Obligation 1 · Obligation 2 · bby et yen (= *benchmark bond yield* et yen) · Commodités et devises · chine, aud, nzd · petrole et usd | ~44 min |
| `sentiment_de_marche` | Risk on · Risk off · Indices de devises · audnzd | ~30 min |

### Module avancé (32 vidéos)

| Sous-module | Vidéos | Durée cumulée |
|---|---|---|
| `banques_centrales` | Fed · BOE · BOJ · Taux implicites · Baisse hawkish · Hausse dovish · Maintient hawkish, dovish | ~47 min |
| `discours_de_banques_centrales` | Articles · sortie de banque centrale · Tonalité de banques centrales · Documents de banques centrales · etude de cas rba | ~46 min |
| `market_internals` | Généralités · Cesi · Ctot · Ctot differentials · Inflation expectations · Inflation expectations differentials · Yield differentials · Risk reversal · Mean reversion · Volatilité implicite | ~29 min |
| `donnees_du_cftc` | Généralités · Interprétation | ~9 min |
| `taux_implicite_maj` | BOJ, étude de cas | ~21 min |
| `volatilite_implicite_maj` | Volatilités implicites Forex | ~8 min |
| `annonces_eco_maj` | Comment interpréter une annonce économique | ~11 min |
| `strategie_de_swing_trading` | Stratégie swing | ~4 min |
| `strategie_de_day_trading` | Stratégie day trading | ~7 min |
| `comment_trader_avec_analyse_fonda` | Buy the rumor Sell the news · Comment faire une bonne analyse fondamentale | ~15 min |
| `ressources_maj` | Ressources | ~13 min |

**Datation du contenu** : le corpus a été enregistré sur plusieurs périodes (indices internes :
données de novembre 2024 dans le module de base ; « on est le 20 novembre » dans *Articles* ;
contexte Trump/tarifs et CPI du 12 février dans les vidéos « mises à jour » ; fin janvier 2025
pour l'étude de cas BOJ). Les chiffres, mandats et membres cités sont donc **datés** — à
re-vérifier avant tout usage factuel.

**Durées** : toutes les valeurs ci-dessus sont recalculées depuis `audit_durees.csv`
(correction du 2026-11-02 [ADDENDUM] ; la version antérieure comportait notamment
`correlations` annoncé ~63 min pour **~44 min** réelles et `market_internals` ~25 min pour
**~29 min** réelles). Totaux vérifiés : module de base 5 h 28 min 55 s, module avancé
3 h 29 min 27 s, corpus complet **8 h 58 min 22 s**.

---

## 2. La méthode centrale : trois vérifications, un biais éventuel

Source principale : `comment_trader_avec_analyse_fonda/Comment faire une bonne analyse
fondamentale` [PROFOND].

1. **« Il n'y a pas de règle »** : contrairement à l'analyse technique, l'analyse fondamentale
   n'a pas de chemin défini ni de règle mécanique « si X alors achète ». La compétence vient de
   l'**accumulation d'expériences vécues au quotidien** (« tu dois être là chaque jour, être
   présent chaque jour, pour écrire ces expériences-là »). → C'est l'argument fondateur du
   journal : il est l'outil qui capitalise ces expériences.
2. **Une bonne analyse = trois éléments vérifiés** :
   - **Fondamentaux purs de la devise** : chiffres macro (2 dernières semaines), banque centrale
     (hawkish/dovish), discours des membres, projections ;
   - **Facteurs externes** : corrélations, géopolitique, risk-on/risk-off, matières premières,
     Chine (pour AUD/NZD) ;
   - **Positionnement et pricing actuels** : où en est la devise graphiquement, que dit le
     tableau des taux implicites (pricé / non pricé), positionnement CFTC, chiffres du jour
     susceptibles de déplacer le pricing.
3. **Conclusion éventuelle** : un **biais** (transcrit « billet ») long terme, moyen terme ou
   court terme — **potentiellement**. « On peut vérifier ces trois éléments et ne pas avoir de
   conclusions… on attend d'autres développements. Souvent, c'est pas clair. »
   → L'incertitude et le « pas encore d'avis » sont des issues légitimes.

## 3. Les deux routines

### 3.1 Préparation de la semaine (week-end)

Source : *Comment faire une bonne analyse fondamentale* + *Articles* [PROFOND].

- **Quand** : samedi et/ou dimanche soir.
- **Avec quoi** : les articles **week ahead** (plusieurs sources, voir §10) + le **calendrier
  économique** de la semaine + Trading Economics pour vérifier les chiffres.
- **Produit** : une synthèse **pour chaque devise importante de la semaine**, structurée sur les
  trois éléments (fondamentaux purs + facteurs externes + positionnement actuel), avec les
  événements attendus et le biais éventuel qui s'en dégage.

### 3.2 Préparation de la journée (quotidien, le matin)

Source : *Articles* [PROFOND — la vidéo la plus opérationnelle du corpus].

- **Quand** : vers **6 h 50 – 7 h**, après la session asiatique, avant la session européenne.
- **Durée** : « 30 minutes à 1 heure, c'est 90 % de la lecture. Ensuite, je passe au
  graphique. »
- **Séquence** :
  1. Lire les articles de préparation (ING FX Daily en premier, puis Newsquawk ×2, BNZ Markets
     Today en option, newsletters Reuters) ;
  2. Déterminer le **sentiment du matin** (risk-on / risk-off / neutre) via les indices de
     devises (Mataf) et le comportement des actifs ;
  3. Consulter le **calendrier du jour** : quelles annonces peuvent **changer** le sentiment ;
  4. Rédiger dans le **carnet fondamental** (l'outil du formateur, origine des carnets PDF de
     l'utilisateur) une synthèse par devise :
     - contexte fondamental **avec biais long / moyen / court terme** (si possible) ;
     - attentes de la journée et impacts probables ;
     - analyse du sentiment actuel (risk-on/off/neutre, session asiatique) ;
     - analyse positionnelle et pricing ;
     - probables opportunités (croisement fonda × stratégie technique).
  5. Passer aux graphiques ; en journée, lire les articles d'analyse/décryptage (ING, FXStreet
     [transcrit « Epic Street », à vérifier], City Index) ;
  6. **Débriefer après la session** : « vous allez faire un debriefing de ce qui s'est passé par
     rapport à ce que vous avez analysé ce matin ». → C'est la boucle de progression.
- **Réflexe de vérification** : « quand les articles donnent des chiffres, prenez l'habitude
  d'aller les vérifier » sur Trading Economics.
- Le formateur précise que le carnet quotidien est un **outil de débutant** : après 1–2 mois,
  « ça fait partie intégrante de vous ». L'application Macro Journal ne doit donc pas imposer un
  formulaire rigide, mais offrir la structure quand on la veut.

## 4. Interprétation d'une annonce économique

Sources : `annonces_eco_maj/Comment interpréter une annonce économique` + `calendrier_eco/
Cas pratiques` [PROFOND].

**Trois étapes** :
1. **Identifier la nature** de l'annonce (inflation, emploi, croissance…) — « interpréter sans
   savoir ce que c'est, c'est absurde » ;
2. **Connaître la théorie** (inflation ↑ → devise ↑, etc.) ;
3. **Remettre dans le contexte fondamental actuel** — « c'est la troisième partie qui va dicter
   comment on se positionne » : partir de la politique monétaire, puis du contexte géopolitique,
   lister les éléments extérieurs, construire des scénarios.

**Trois questions de contextualisation** :
- Q1 — que fait actuellement la banque centrale, et quel chiffre veut-elle ?
- Q2 — comment les chiffres récents sont sortis par rapport à ce but ?
- Q3 — qu'en pense le marché, et comment est-il déjà positionné ?

**Typologie des impacts** : court terme (chiffre isolé à contre-courant → rebond éphémère s'il
n'est pas confirmé) ; moyen terme (2–3 semaines de chiffres concordants opposés au
positionnement → repositionnement/pricing) ; long terme (ton clair de la BC + convergence
prolongée des chiffres, éventuellement mêlé de géopolitique).

**Règles transversales** :
- « On veut que tous les chiffres racontent la même histoire » (*Emploi*) : plus le paquet est
  cohérent, plus la réaction est violente ; des chiffres contradictoires affaiblissent
  l'interprétation.
- Le consensus est une **médiane/moyenne de sondages d'institutions** (JP Morgan, Goldman
  Sachs, SocGen…), avec des bornes (écarts-types) ; « prédire le chiffre exact, ce n'est pas
  possible » — préparer **les deux scénarios** en amont (*Comment sont trouvées les prévisions*).
- Saisonnalité : surprises d'inflation à la hausse quasi systématiques en janvier aux US
  (facteurs saisonniers) → le « meilleur scénario » peut être la surprise contraire (*Cas
  pratiques*).
- Un chiffre allant **contre** le contexte produit une volatilité ponctuelle (scalp/day), pas
  un renversement de tendance.
- « On ne peut pas faire de l'à-peu-près en fondamentale, on doit être précis, ultra précis. »

## 5. Indicateurs étudiés

### Inflation [PROFOND — le sous-module le plus dense]
- Globale vs **core** (exclut nourriture + énergie, « pour ne pas biaiser les calculs »).
- **Mesures préférées par banque centrale** : Fed → **PCE / Core PCE** ; BCE → **HICP/IPCH**
  (préliminaire puis finale ~2 semaines après : « le marché réagit toujours au préliminaire ») ;
  BoC → **CPI trim / median / common**, la core suivie = moyenne des trois ; UK → **inflation
  des services** (indicateur clé de la BoE, absente des calendriers gratuits) ; NZ →
  tradable/non-tradable (données trimestrielles).
- Zone euro : lire d'abord France, Allemagne, Espagne pour anticiper le HICP global.
- Cible ~2 % « universelle » (fourchette 1–3 % pour BoC et RBNZ).
- Déflation (territorité négative, cercle vicieux japonais) vs désinflation [ÉVOQUÉ puis
  développé dans `module_avance/banques_centrales/BOJ`].

### Emploi [PROFOND]
- **Le paquet de base, toujours** : nombre d'emplois créés, taux de chômage, salaires
  (« ces trois éléments ne vont jamais manquer »).
- US : **NFP** (1er vendredi du mois), average hourly/weekly earnings, **JOLTS** (openings,
  quits), **ADP** (secteur privé), **initial/continuing jobless claims** (hebdo ; neutraliser
  les chocs climatiques type ouragan).
- Canada/Australie : distinction **temps plein / temps partiel** (« toujours mieux si la
  majorité des jobs sont à temps plein »).
- NZ : trimestriel.

### Croissance / PIB [PROFOND]
- « Pas une annonce qui crée de la volatilité… un élément dont on tient compte dans nos
  analyses. » Important « quand c'est vraiment catastrophique » (cas Europe).
- **GDPNow** (Fed d'Atlanta) : prévision du PIB US en avance → peu de surprise à l'annonce.
- Estimations multiples : flash/advance → second → third estimate.
- PIB faible durable (plusieurs trimestres) → pèse sur l'arbitrage de la BC vers la baisse des
  taux.

### PMI [PROFOND]
- Indice des directeurs d'achat ; **> 50 expansion, < 50 contraction** ; les meilleures
  réactions viennent des **grosses déviations** et du **passage d'une zone à l'autre**.
- Indicateur **avancé** via ses composantes : new orders (production future), employment (NFP
  futur), prices paid (inflation future), business activity.
- Sources : **ISM** (US, référence), **S&P Global** (HCOB en Europe, « flash » = 1re
  estimation, composite), **Caixin** (Chine, transcrit « Kexin »).
- France + Allemagne = « les deux puissances économiques majeures en Europe », le PMI zone euro
  suit leurs tendances.

### PPI [PROFOND]
- Indice des prix à la production : chaîne usine → grossiste → consommateur (CPI) ;
  global vs core (exclude energy & food ≠ core si on exclut autre chose) ; indicateur avancé de
  l'inflation mais « donne de faux signaux », « pas un élément que je trade ».

### Taux d'intérêt [PROFOND]
- « Taux élevés ⇒ devise forte ; taux bas ⇒ devise faible. C'est le principe ultime de
  base », via la transmission BC → banques commerciales → emprunts/consommation → rareté de la
  monnaie.
- BCE : trois taux, celui qui compte en forex = **deposit facility rate**. Fed : Fed Funds
  Rate (borne haute de la fourchette). RBNZ : **OCR**.
- Déroulé d'une annonce : taux → communiqué (« dans la minute ») → projections (Fed : tous les
  3 mois) → **conférence de presse ~30 min après**.

### Classification des annonces (calendrier)
- **Majeures** (volatilité) : taux, inflation, emploi, PMI.
- **Secondaires** : PIB, retail sales, PPI, ZEW/IFO/Michigan, jobless claims — quasi zéro
  volatilité isolée, utiles en analyse.
- **Tertiaires** : importations/exportations, logement, balance commerciale — « je ne trade
  jamais ce genre de news… seulement intéressant en termes d'analyse » (notamment Chine, US).

## 6. Suivi des banques centrales [PROFOND]

- **« En fondamental, tout est lecture »** : lecture, interprétation, analyse des documents
  officiels ; « si vous n'aimez pas lire, c'est pas la peine » (*Étude de cas BOC*).
- **Documents par réunion** : statement/communiqué → projections (SEP Fed trimestriel, dot
  plots) → **minutes ~3 semaines après** (« le vrai sentiment du débat ») ; BoE : **Monetary
  Policy Summary + vote des 9 membres + minutes publiés en même temps** ; BoJ : statement +
  projections + **Summary of Opinions** (~2 semaines) ; Beige Book (Fed, module annoncé).
- **Calendriers** : calendrier économique (discours avec sujets) + calendriers officiels des
  BC. **Filtrer par sujet** : un discours sur « les banques et l'IA » n'intéresse pas le forex.
- **Tonalité** [PROFOND — `Tonalité de banques centrales` + `etude de cas rba`] :
  - grilles d'éléments de langage hawkish / dovish / prudent / neutre (PDF du formateur) ;
  - le ton se juge **en relatif d'une réunion à l'autre** et par rapport aux chiffres ;
  - **asymétrie clé** : un membre hawkish qui « lâche » du dovish = signal fort (et inversement) ;
  - détection des **apparitions/disparitions de formulations** (ex. RBA 18 juin → 24 sept.
    2024 : « le conseil a discuté de relever les taux » → « n'a pas explicitement envisagé une
    hausse » = virage dovish détectable avant le mouvement).
- **Matrices décision × tonalité** [PROFOND] : la devise réagit au couple
  (décision × discours × pricing), jamais au titre seul :
  - baisse hawkish → chute initiale puis retournement violent à la hausse possible ;
  - hausse dovish / inférieure au pricing → chute (cas BoJ 19/03/2024 : sortie du négatif « sans
    perspective » → yen baisse parce que le marché est déçu) ;
  - maintien + durcissement du langage → hausse (cas RBA 18/06/2024, « l'AUD a explosé »).
- **Blackout Fed** (~2 semaines avant réunion) : la Fed « fait fuiter » via les
  insiders/whisperers ; **Nick Timiraos** (Wall Street Journal) = « journaliste le plus proche
  de la Fed » — « ne pas y être abonné, c'est un crime ».
- Membres/tonalités cités (datés fin 2024, [À VÉRIFIER]) : Schnabel (BCE, hawkish), Mann
  (BoE, hawkish), Bailey (BoE), Ueda (BoJ), Tamura/Takata (BoJ, hawkish), Mimura
  (« diplomate de la devise », en réalité ministère des Finances), Schlegel (SNB), Macklem
  (BoC), Bullock + Hauser (RBA), Orr (RBNZ, dates douteuses), Powell (Fed), Lagarde (BCE,
  « faire du Lagarde = parler pour ne rien dire »).
- **Outils de politique monétaire** : taux (favori), **réserves de devises étrangères**
  (mécanique SNB : réserves ↑ = affaiblir le franc), achats/ventes d'obligations (QE).
- La BoJ « travaille beaucoup avec le gouvernement » (contre-exemple à l'indépendance).

## 7. Positionnement, pricing et market internals [PROFOND]

### Taux implicites (le cœur quantitatif)
- Définition : « le taux du marché » (continu) vs taux officiel fixé en réunion.
- Lecture du tableau : implied rate par réunion, **pb cumulés** (combien de mouvements de 25 pb
  sont pricés d'ici chaque échéance), pb ajustés par réunion, scénario le plus probable.
- **Probabilités** : pb pricés ÷ 25 (au-delà de 25 : ÷ 50) → % de probabilité.
- **Price in / price out** : le marché price selon **les chiffres**, pas selon les seules
  projections de la BC ; mesurer **l'écart marché ↔ guidance** pour repérer les exagérations
  dans les deux sens (cas Fed 18/12/2024 : marché à −70 pb fin 2025 vs guidance 2 baisses →
  pricing out massif → dollar s'apprécie « massivement »).
- Méthode matinale : relever les changements vs la veille, chercher le **catalyseur** (la
  donnée qui corrobore la banque contre le marché), croiser avec CFTC et narratif.
- BoJ : historiquement incréments de 10 pb ; depuis 2024, mouvements de 20/15/25 pb → le
  formateur a **refondu son calcul au seuil de 25 pb**. « Cet outil-là, je ne peux pas faire
  mes analyses sans ça. »
- Sources : CME FedWatch (Fed seulement, gratuit), tableau global (outil du formateur, non
  public), articles ING (mentionnent les pb pricés).

### CFTC / COT [PROFOND]
- Commercials (couverture) vs **non-commercials (spéculateurs — ce qui nous intéresse)**.
- Publication le **vendredi**, données arrêtées au **mardi** → retard de 3 séances à intégrer
  (« un événement d'aujourd'hui n'est pas dans les données de ce vendredi »).
- Lecture : classement des devises par positions acheteuses/vendeuses ; zones **80–100 =
  suracheté**, bas du classement = survendu ; **USD aggregate** (non pondéré) privilégié vs
  DXY (pondéré, euro 57,6 %) ; niveau institutionnel (données CME, « ligne rouge »).
- Logique **ressort** : extrême + catalyseur inverse → repositionnement violent. Jamais un
  signal autonome : « voir comment le marché est positionné actuellement sur une devise ».
- **Provenance [OUTIL PERSO, pas FORMATION]** : la normalisation du Net Non-Commercial en
  % de l'Open Interest et les percentiles P20/P80 sur 52 semaines sont une implémentation
  personnelle de l'utilisateur (moteur Python, voir `COT_ENGINE.md`), postérieure à la
  formation. La formation enseigne un **rang transversal** des devises (zones 80–100 =
  suracheté) et privilégie l'**agrégat USD non pondéré** ; le moteur utilise pour l'USD le
  contrat ICE U.S. Dollar Index (DXY pondéré — proxy documenté). Les deux familles de
  mesures (rang transversal enseigné vs percentile historique calculé) sont documentées
  séparément et ne doivent jamais être confondues (`COT_ENGINE.md` §8).

### Market internals & différentiels
- **Règle transversale des « differentials »** : pour toute paire, comparer l'indicateur des
  deux devises ; la paire est *censée* suivre le différentiel.
- **CESI** (surprises économiques agrégées) : comparaison des 8 pays sur un même graphique →
  devises à acheter/vendre fondamentalement ; une devise peut rester déconnectée longtemps.
- **CTOT** (terms of trade) ; **inflation expectations** (5 ans/10 ans, positivement corrélées
  à la devise) ; **yield differentials** (2 ans/10 ans) ; **risk reversal 1 mois** (même
  tendance que la paire ; théorie des options renvoyée à un module non livré) ;
  **volatilité implicite** + **mean reversion** (loi normale, canal ±2 écarts-types ≈ 95 % du
  temps ; VI « chère » au-delà de la borne → signal de retour à la moyenne ; corrélation
  inverse prix/VI « dans 90 % des cas » [À VÉRIFIER] ; exceptions USD/CAD, USD/MXN).
- Deux sens d'usage : **confirmation** d'une thèse fonda+technique, ou **screening inversé**
  matinale des grosses décorrélation/mis-pricing → puis analyse fondamentale. « On peut faire
  ça chaque matin. »

## 8. Corrélations et facteurs externes [PROFOND]

- **Obligations** : relation inverse prix/rendement ; adjudications (bid to cover > 2 = très
  bien, comparer aux 12 précédentes) ; **rendements 10 ans = baromètre du sentiment**
  (« le rendement à 10 ans, c'est la base du tout ») ; **courbe inversée** (10 ans < 2 ans) =
  « signe avant-coureur de récession depuis 1950 » [À VÉRIFIER — affirmation non sourcée].
- **Benchmark bond yield vs JPY** (« bby et yen ») : moyenne des rendements 10 ans de 7
  zones (US, DE, CA, CH, AU, NZ, GB — Japon exclu) ; corrélation inverse avec le yen (carry
  trade) ; « les rendements 10 ans américains suffisent largement » en simplification.
- **Commodity currencies** : AUD ↔ cuivre (secteur minier ~27,5 % du PIB, exports ~25 % dont
  ~65 % énergie/minier [chiffres à vérifier]) ; CAD ↔ pétrole WTI ; NZD ↔ matières premières +
  **Chine** (28 % de ses exportations). « Ces corrélations ne sont pas parfaites. »
- **Chine → AUD/NZD** : annonces de la BCE chinoise (soutien à l'économie) = catalyseur
  haussier ; surveiller « tout le temps » avant d'analyser AUD/NZD.
- **Pétrole → USD** : enseigné comme corrélation **positive** (pétrole ↑ → anticipations
  d'inflation ↑ → rendements 10 ans ↑ → USD ↑) — **[À VÉRIFIER]** : va à l'encontre de la
  lecture classique (pétrole coté en USD) et n'est pas arbitrée avec le cas CAD producteur.
- **Risk-on / risk-off / neutre** : risk-on → AUD, NZD, CAD ↑, rendements ↑, indices ↑,
  cuivre ↑ ; risk-off → USD, CHF, JPY ↑, rendements ↓, or/pétrole ↑ (tensions géopolitiques) ;
  « la géopolitique est un catalyseur très rapide » ; le risk-off « plus facile à trader car
  soutenu par des fondamentaux » ; cas de l'USD flat en risk-off quand la mauvaise nouvelle
  vient des US.
- **Indices de devises** : DXY pondéré (euro 57,6 %, pas d'AUD/NZD) vs **indices non pondérés**
  (Mataf, gratuit, temps réel ; construction TradingView en produit de paires inversées) ;
  « devises en haut = performantes » → lire le sentiment avant même les articles.
- **AUD/NZD** : « ma paire préférée, parce que fondamentalement c'est une paire qui respecte
  beaucoup les fondamentaux » — deux devises du même groupe → chocs externes neutralisés →
  moteur = **différentiel de politique monétaire RBA/RBNZ** (fondamentaux purs).

## 9. Stratégies du formateur (contexte, pas un modèle à copier)

- **Risk event trader** (« c'est mon style de base ») : trader les annonces économiques au
  quotidien ; swing/day = seulement 20–30 % des positions mensuelles.
- **Swing** : scénario idéal à 5 éléments (fondamentaux purs déterminés + facteurs externes
  alignés + positionnement favorable avec marge + market internals + confirmation graphique) ;
  **minimum indispensable : fondamentaux purs + positionnement + confirmation graphique**. Ne
  pas re-vérifier tout si le biais sur l'autre devise de la paire va dans le même sens.
- **Day trading** : base = **sentiment du marché à l'instant T** (au-delà des fondamentaux) ;
  vérifier le calendrier pour les annonces pouvant inverser le sentiment ; confirmation
  technique en 30 min ; TP courts.
- **Buy the rumor, sell the news** : le marché se positionne **par anticipation** ; « il faut
  que ce soit pricé dans le prix » ; une annonce conforme au pricing ne produit pas de
  mouvement ; une annonce contraire produit un mouvement violent dans le sens contraire.
- Gestion type : sortie partielle à 50 %, SL déplacé en breakeven+, « vite couper les pertes ».
- AT volontairement minimaliste : supports/résistances et figures, « le déclencheur, c'est le
  fondamental ».

## 10. Sources utilisées par le formateur

| Type | Sources citées (transcription → probable) |
|---|---|
| Calendriers/données | Trading Economics (« je n'ai pas trouvé mieux gratuit »), Investing.com (3 étoiles), calendrier personnel payant |
| Articles quotidiens | **ING FX Daily** (« la base de la base »), **Newsquawk** (2 articles/jour : pré-européenne ~6h50, pré-US), BNZ Markets Today (PDF), **Reuters** (Daily Briefing + MorningBid US/Europe/Asia), Monex [?] |
| Week ahead | Newsquawk Week Ahead (vendredi), Westpac IQ (AUD/NZD), Scotiabank (CAD), Société Générale Research [transcrit « Sepp/Seb Group », à vérifier], banque suisse [UBS ?], Monex |
| Spécialisés | Nordea + [Swedbank ?] (devises nordiques + taux implicites Riksbank/Norges), FXStreet [« Epic Street »], City Index |
| Banques centrales | Sites officiels (Fed, BCE, BoE, BoJ, SNB, BoC, RBA, RBNZ), chaînes YouTube, Instagram, transcriptions PDF des conférences |
| Positionnement | CFTC (cftc.gov), Financial Juice (flux temps réel + force des devises du matin) |
| Taux implicites | CME FedWatch (Fed), tableau global toutes BC (outil privé de la formation) |
| Recherche institutionnelle | **FXWatcher** (abonnement ~75 €/mois : Goldman Sachs, Citi, etc. via Google Drive quotidien) — payant |
| Twitter/X | Nick Timiraos, Financial Juice, Bloomberg, Newsquawk, IG France, Michael Brown (Pepperstone), « Timmy Rose » [?] |

Le formateur insiste : « il y a trop de sites… c'est à chacun de définir sa routine » ;
l'essentiel est une **routine stable** sur un ensemble choisi.

## 11. Ce que la formation ne fait PAS (garde-fous pour Macro Journal)

- Aucun **score** ou notation synthétique par devise ; aucune addition de points.
- Aucune règle mécanique d'entrée/sortie en fondamental ; pas d'automatisation.
- Pas de moteur de probabilités maison : les taux implicites sont **lus** depuis des sources
  externes (CME, tableaux, articles), jamais recalculés.
- Le marché des options (risk reversal, VI) est utilisé comme **lecture graphique**, sans
  théorie complète (module « options » annoncé, non livré dans le corpus).
- Pas de scraping, pas d'agrégation automatique : « tout est lecture ».
- Les nuances sont systématiques : « les corrélations ne sont pas parfaites », « ça donne de
  faux signaux », « vous n'allez pas retrouver la même chose deux fois », « le retour à la
  moyenne, ce n'est pas un effet magique ».

## 12. Glossaire des erreurs de transcription récurrentes (Whisper)

`la fête`→la Fed · `au quiche/Aokish/haokish`→hawkish · `douviche/d'Ovich`→dovish ·
`tout l'intérêt`→taux d'intérêt · `CFSI/saisies/seizi`→CESI/CFTC · `billet`→biais ·
`espérences`→expériences · `natifs`→actifs · `divisoire/doula`→devise/dollar ·
`Caripride/carré trade`→carry trade · `ma taffe`→Mataf · `bby`→benchmark bond yield ·
`Newsquoke/Inusco`→Newsquawk · `Epic Street`→FXStreet [à vérifier] · `Final Show Juice`→
Financial Juice · `Kexin`→Caixin · `bit to cover`→bid to cover · `carnaval`→canal ·
`perte/paix`→paire · `SP505`→S&P500 · `l'écor`→le core · `Timmerose`→Timiraos ·
`réseaux de devises`→réserves de devises.

## 13. Points nécessitant une vérification externe (extraits)

1. Corrélation pétrole→USD enseignée **positive** (contredit la lecture classique inverse ;
   tension interne non arbitrée avec CAD producteur).
2. « Depuis 1950, quasi toutes les inversions de courbe → récession » (non sourcé).
3. « Agriculture = 20 % des travailleurs US » (probable 2 % ; déformation probable).
4. Mandats/membres datés : Lagarde « 2017 » (lire 2027), gouverneur RBNZ (Orr, dates
   incohérentes), dates Bullock, composition des réserves SNB, « maximum 5 réunions/an » SNB
   (en pratique 4).
5. Définition du CTOT (soustraction vs ratio, deux définitions coexistent dans la vidéo).
6. Fenêtre CFTC décrite « mercredi S-1 → mardi S » (calendrier officiel : positions arrêtées
   au mardi de la semaine de publication).
7. « 90 % des cas » de corrélation inverse prix/VI ; « 70 % » vs « 10 % » dans la loi normale
   appliquée à la VI (incohérences internes).
8. Chiffres Australia/NZ (parts minière, exports) déformés par la transcription.

Ces points sont conservés **comme enseignés**, marqués [À VÉRIFIER] : une vérification externe
(Reuters, sites officiels des BC, FRED) permettrait de les corriger — mais la correction doit
rester **explicite et datée**, jamais silencieuse.

---

## 14. Implications directes pour Macro Journal

| Élément de méthode | Traduction applicative (principe, pas formulaire) |
|---|---|
| Trois vérifications (fonda purs / externes / positionnement) | gabarits de notes et sections des dossiers devise et paires |
| Biais LT/MT/CT **optionnel** (neutre = issue valide) | champ biais facultatif à 4+ valeurs (haussier/baissier/neutre/indéterminé) |
| Routine hebdo (week ahead + calendrier + synthèse par devise) | module Weekly Outlook structuré sur le même flux |
| Routine quotidienne (articles → sentiment → synthèse → graphique → **débrief**) | module Journal + modèle de note quotidienne ; le débrief = future boucle Review |
| « Tous les chiffres racontent la même histoire » | fiche événement avec réel/consensus/précédent + champs interprétation **séparés** |
| Tonalité en relatif réunion après réunion, apparition/disparition de formulations | Narrative Tracking avec versions datées et motif de changement |
| Differentials : comparer deux devises sans dupliquer | Pair Research reliant les dossiers des deux devises |
| « En fondamental, tout est lecture » + vérifier les chiffres | gestion des sources avec URL, date de publication et date de consultation |
| Sources externes existantes (CME, CFTC, TE, ING…) | raccourcis et enregistrement de liens, jamais de reconstruction |
| Erreurs de transcription du corpus | les documents de référence restent corrigibles et datés (pas de vérité figée) |
| Positionnement COT chiffré (% OI, percentiles, rangs) | [OUTIL PERSO] moteur Python existant, importé en v0.6 — observations structurées, indicateur jamais signal (`COT_ENGINE.md`) |

---

## 15. Provenance des outils et implémentations personnelles [ADDENDUM, 2026-11-02]

Distinction désormais explicite entre trois strates — exigée par l'addendum, qui constatait
que ce document pouvait laisser croire que tout le socle méthodologique chiffré venait de la
formation :

1. **Ce qui vient de la formation** : l'intégralité des §1 à §13 — méthode à trois
   vérifications, routines hebdo/quotidien, interprétation des annonces, suivi des banques
   centrales, lecture « manuelle » des rapports CFTC (classement des devises, zones 80–100,
   retard mardi → vendredi), market internals lus sur des sources externes. Y compris le
   tableau global des taux implicites cité au §7, qui est un **outil privé du formateur**.
2. **Les outils développés indépendamment par l'utilisateur [OUTIL PERSO]** : le moteur
   Python COT (`cot_reports`, Legacy Futures) — téléchargement, extraction des 8 marchés,
   calcul du Net Non-Commercial % OI, percentiles 52 semaines, dashboard matplotlib.
   Rien de tout cela n'est enseigné ni fourni par la formation, et rien ne doit lui être
   attribué. Deux scripts ont été audités intégralement (voir `COT_ENGINE.md`).
3. **Les améliorations apportées à notre propre méthode [AMÉLIORATION]** (postérieures à la
   formation, assumées comme telles) : percentiles réellement glissants « as-of »
   (correction du biais temporel des scripts initiaux), distinction documentée percentile
   historique / rang transversal, journalisation des anomalies d'import, étiquetage du
   proxy DXY.

**Point à clarifier avec l'utilisateur** : le commentaire `# Style ForexFondamentale :
lignes droites` présent dans l'un des scripts Python suggère que la présentation « lignes
droites » des percentiles imite un style vu ailleurs (la formation ? un autre auteur ?). À
date, la provenance de ce choix stylistique n'est pas établie ; elle n'est attribuée à
personne et reste à documenter.
