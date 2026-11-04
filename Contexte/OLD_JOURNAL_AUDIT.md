# OLD_JOURNAL_AUDIT — Audit des anciens carnets fondamentaux

> **Documents examinés** : `CARNET FONDAMENTAL DE FEVRIER.pdf` (8 pages) et
> `CARNET FONDAMENTAL DE MARS.pdf` (7 pages), format letter paysage, texte extrait avec
> conservation de la mise en page (pdftotext -layout) + lecture du contenu intégral.
> **Cadre** : ces carnets ne sont **pas un modèle d'interface à reproduire**. Ils documentent
> ce qui était jugé important, le niveau de détail et la logique de surveillance. L'objectif
> est de conserver les bonnes intuitions et de dépasser les limites du format tabulaire.

---

## 1. Structure constatée

Grille mensuelle : **une ligne par économie, une colonne par thème**.

| Colonne | Contenu | Exemple (USD, février) |
|---|---|---|
| TAUX | taux directeur | 4,50 % |
| INFLATION — CPI général | + mesure préférée entre parenthèses pour l'USD | 3,00 % (PCE 2,40 %) |
| INFLATION — CPI core | idem core | 3,30 % (PCE 2,80 %) |
| ECONOMIE — PIB | dernière valeur | 2,50 % |
| EMPLOI — Chômage | + NFP entre parenthèses pour l'USD | 4,00 % (NFP 143 k) |
| PMI — Manufacturier / Services | dernières valeurs | 51,6 / 49,7 |
| DATAS | **journal daté** des publications du mois : `jj/mm: Indicateur (réel att consensus préc précédent)` | `-07/02: NFP (143K att 170K préc 307K)` |
| DISCOURS | citations/paraphrases de membres de BC, notes de marché | Bowman, contrats à terme Fed, Trump/Mexique |

**Lignes** (ordre identique sur les deux mois) : USD, EUR, JPY, CAD, GBP, CHF, AUD, NZD,
**Chine**. Les devises ne sont pas identifiées par du texte mais par la position/couleur de la
ligne (le code devise n'apparaît pas dans la couche texte du PDF).

**Points structurels notables** :
- La **Chine est une 9ᵉ ligne** (taux 3,10 % = LPR, PMI Caixin, inflation, retail sales,
  chômage) — traitée comme **facteur externe** pour AUD/NZD, exactement comme l'enseigne la
  formation (`chine, aud, nzd`). L'application doit pouvoir représenter ce cas.
- L'USD porte ses **mesures préférées** (PCE) en plus du CPI — même logique que le module
  Inflation (mesure préférée par BC).
- La colonne DISCOURS mélange : propos de membres (Bowman, Tamura, Mann, Bullock, Uchida),
  lecture de marché (contrats à terme sur les taux), et politique commerciale (Trump/Mexique).

## 2. Ce que le format révèle de la méthode personnelle

- **Couverture systématique des 8 économies**, comparables colonne à colonne : c'est déjà de
  l'**analyse relative implicite** (le tableau se lit en lignes ET en colonnes).
- Choix d'indicateurs **aligné sur la formation** : taux, CPI général/core, PIB, chômage, PMI
  manu/services = les annonces majeures + secondaires du référentiel.
- Le journal DATAS enregistre systématiquement le **triplet réel / attendu / précédent** —
  c'est-à-dire la notion de **consensus et de surprise**, cœur de la méthode d'interprétation.
- Traçage des **décisions de taux** dans les DATAS (`-06/02: Baisse de taux de 25 pbs`) et des
  discours dans DISCOURS : le suivi de la politique monétaire est bien présent.
- La mise à jour est **continue dans le mois** (le carnet de février contient des datas du
  31/01 au 27/02) : c'est un journal vivant, pas une photo fixe — mais figé à la fin du mois.

## 3. Points forts à conserver (intuitions justes)

1. La **grille comparative multi-devises** : un coup d'œil = qui est fort, qui est faible.
2. Le **triplet réel/consensus/précédent** pour chaque publication datée.
3. La distinction **globale / core** (et PCE pour l'USD, mesures spéciales CAD : common/trim/
   median sont d'ailleurs cités dans les DATAS de février).
4. La **Chine comme ligne externe** rattachée à la surveillance AUD/NZD.
5. Les **discours de membres** notés avec leur propos — matière première du suivi de
   tonalité.
6. Le suivi des **décisions de taux** au fil du mois.
7. La **précision des sources implicites** : les intitulés montrent la source (SPG vs ISM,
   Caixin, HCOB, Tokyo CPI…) — la connaissance « quelle mesure pour quelle BC » est déjà là.
8. L'**actualisation mensuelle régulière** : deux mois consécutifs tiennent.

## 4. Limites structurelles du format (ce que l'application doit dépasser)

| # | Limite constatée | Conséquence |
|---|---|---|
| 1 | **Instantané mensuel figé** : chaque carnet repart d'une copie du précédent | pas d'historique continu ; on ne peut pas retracer ce qu'on pensait le 12/02 |
| 2 | **Aucune colonne biais/narratif** : le pilier 1 (fonda propres) est couvert, la Chine couvre un bout du pilier 2, mais le **pilier 3 (positionnement/pricing) est entièrement absent** | l'analyse ne capitalise pas sur la méthode à trois vérifications de la formation |
| 3 | **Aucune interprétation** : des faits (chiffres, propos) sans lecture, sans scénario, sans conditions d'invalidation | impossible de reconstituer un raisonnement |
| 4 | **Aucune source/URL/date de consultation** | rien n'est vérifiable ni re-trouvable |
| 5 | **Lignes périmées** : en mars, la ligne NZD est strictement identique à février (4,25 % / 2,20 % / 2,70 % / −1,50 % / 4,80 % / 51,4 / 50,4, aucune data) | le format statique tolère silencieusement l'oubli |
| 6 | **Saisie manuelle fautive** : `Reteil Sales`, `infaltion`, `Servcices`, `20:02:`, `-09:03:`, `-03/: I a (47,5 att 8, préc 49)` (chiffres perdus) | l'erreur se propage sans contrôle |
| 7 | **Unités/protocoles implicites** : « GDP YoY (2,8 %) » sans préciser annualisé QoQ vs YoY ; PIB NZD « −1,50 % » sans période ; certaines valeurs invérifiables | l'audit futur des propres analyses est impossible |
| 8 | **Aucun lien** entre un événement, l'analyse qu'il a nourrie et le narratif qu'il a fait évoluer | la mémoire ne se construit pas |
| 9 | **Aucun débrief/review** : pas de comparaison anticipations vs réalisé | la boucle d'apprentissage enseignée par la formation (debriefing quotidien) n'existe pas |
| 10 | **Duplication structurelle** : le même indicateur resaisi chaque mois ; EUR/USD n'est jamais analysé en tant que paire | travail répétitif, pas d'analyse relative explicite |
| 11 | **Fragilité du support** : PDF à texte positionné (l'extraction croise les colonnes), identification des devises par couleur/position | non requêtable, non recherchable |
| 12 | **Format hérité de l'outil du formateur** (« ce carnet fondamental là… c'est pour le daily », vidéo *Comment faire une bonne analyse fondamentale*) : le carnet du formateur est quotidien avec debrief, l'usage ici est mensuel | la pratique a dérivé vers un snapshot mensuel, perdant la boucle de progression |

## 5. Anomalies et points de qualité de données relevés

- **Février** : `20:02: PPI MoM` (date) ; `-03/: I a (47,5 att 8, préc 49)` (CHF, chiffre
  tronqué) ; `Reteil Sales` ; `infaltion` ; deux fois `PPI YoY` le même jour en mars US
  (13/03) sans distinction global/core ; `-03/02: GDP` fragmenté par l'extraction.
- **Mars** : ligne NZD intégralement périmée (copiée-collée) ; `-09:03:` (séparateur inversé) ;
  `Servcices` ; GBP réduit à 4 datas ; CHF à 5 ; aucun DISCOURS sauf Uchida (JPY).
- **Incohérences de périmètre** : le NFP apparaît dans la colonne chômage de l'USD mais pas
  pour CAD/AUD (employment change dans les DATAS) ; le PIB CAD passe de 1,50 % (fév.) à
  1,00 % (mars) sans unité claire ; USD PIB 2,50 % (annualisé implicite) vs EUR 0,90 % (YoY
  implicite) — **les colonnes ne sont pas homogènes entre pays**, ce qui fragilise la
  comparaison ligne à ligne.
- Ces anomalies **ne sont pas des fautes de fond** : ce sont les symptômes prévisibles d'une
  saisie manuelle sans contraintes de saisie. Une fiche événement structurée (champs dédiés,
  contrôles légers) les élimine en grande partie.

## 6. Décisions pour Macro Journal

| Élément de l'ancien carnet | Décision |
|---|---|
| Grille 8 devises + colonnes d'indicateurs | **Dépasser** : remplacée par les dossiers Currency Research alimentés en continu (plus de snapshot figé) |
| 9ᵉ ligne Chine | **Conserver** : table `currencies` avec rôle `external` (CNY) ; notes et événements liables à la Chine |
| Journal DATAS daté (réel/att/préc) | **Conserver** : entité `events` structurée, champs chiffrés facultatifs |
| Colonne DISCOURS | **Conserver** : événements de catégorie `discours` + citation/interprétation + lien vers le narratif |
| Triplets global/core + mesure préférée | **Conserver** : catégorie d'événement + champ `unit`/`measure` explicite |
| Absence de biais, narratif, positionnement, sources, review, historique | **Ajouter** : c'est précisément l'objet des modules Narrative Tracking, sources reliées, note_revisions et Review |
| Copie mensuelle du fichier | **Supprimer** : persistance unique + historique versionné |

**Conclusion de l'audit** : l'ancien journal capture correctement les **faits du pilier 1**
et un fragment du pilier 2, mais ne capture ni le **raisonnement**, ni le **pilier 3**, ni
**l'histoire**. Macro Journal doit conserver la discipline de couverture (8+1 économies, le
triplet de publication, les discours) tout en déplaçant la valeur vers ce qui manque :
interprétation séparée des faits, narratifs versionnés, sources datées, boucle de review.
