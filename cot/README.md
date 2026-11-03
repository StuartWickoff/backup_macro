# Prototype d'itération COT

Ce prototype reste isolé dans `cot/` : il ne modifie pas le dashboard Macro Journal.
Les scripts précédents restent conservés dans `script_trouve_pendant_mes_recherches/`.

## Préparer l'environnement

```bash
python3 -m venv .venv
.venv/bin/python -m pip install -r requirements.txt
```

## Générer des graphiques

Télécharger et tracer les rapports CFTC Legacy Futures :

```bash
.venv/bin/python cot_dashboard.py \
  --start-year 2021 \
  --end-year 2026 \
  --smooth \
  --web-assets-dir ../apps/web/public/cot
```

Les rapports annuels sont conservés dans `.cache/cftc/`; les années historiques
disponibles en cache sont réutilisées. L'année la plus récente est toujours
retéléchargée; si ce téléchargement échoue et qu'un cache existe, il est réutilisé
avec un avertissement. `--refresh-cache` force le rafraîchissement de toutes les années.
Avec `--web-assets-dir`, les graphiques et leur manifeste (date la plus récente,
marché et statut de fraîcheur) sont également publiés dans les ressources du dashboard.

Le dashboard lance automatiquement cette mise à jour avec `./run-dev.sh`.
Pour actualiser les ressources à la main (notamment avant un build de production) :

```bash
./cot/update.sh
```

Comparer avec l'état des courbes de référence datées du 14 septembre 2026 :

```bash
.venv/bin/python cot_dashboard.py \
  --start-year 2021 \
  --end-year 2026 \
  --as-of 2026-09-08 \
  --smooth
```

Les images et le CSV sont écrits sous `output/`; les sorties générées et le cache ne
sont pas suivis par Git.

Ou traiter un CSV déjà téléchargé :

```bash
.venv/bin/python cot_dashboard.py --csv rapport.csv --output-dir output
```

Les marchés sont sélectionnés par nom exact, les positions nettes non commerciales
sont divisées par l'open interest, et les dates en double sont refusées plutôt que
mélangées. Le lissage ne concerne que le tracé : les observations hebdomadaires ne
sont pas modifiées. Le contrat CFTC USD Index est identifié comme proxy DXY pondéré.
Le graphique USD du dashboard est un proxy COT pondéré façon DXY, estimé en comparant
les séries COT à l'image USD Aggregate de la formation :

```text
USD COT proxy = −(0.576 × EUR Net % OI
                 + 0.136 × JPY Net % OI
                 + 0.119 × GBP Net % OI
                 + 0.091 × CAD Net % OI
                 + 0.036 × CHF Net % OI)
```

Les coefficients sont les poids DXY connus pour ces cinq devises et totalisent 95,8 %.
Le poids SEK de 4,2 % est exclu sans renormalisation, faute de série COT SEK trouvée.
AUD et NZD ne sont pas inclus. Cette formule approximative suit la forme de l'image de
la formation, mais n'est ni une série USD publiée par la CFTC ni une formule confirmée
par le formateur. Les dates sans observation exploitable sur un des cinq marchés sont
exclues.

Chaque exécution exporte également `cot_observations.csv`, avec les observations
CFTC et les P20/P80 des 52 dernières observations. La date de référence CFTC est
utilisée pour `--as-of` (mardi), pas la date de récupération ni la date de publication.
Les graphiques affichent les cinq dernières années ; les percentiles restent calculés
sur les 52 dernières observations, indépendamment de cette fenêtre visuelle.

## Seuils et méthode

Par défaut, le graphique affiche la courbe, la zone zéro, et les lignes verte/rouge
P20/P80 calculées à partir des 52 dernières observations disponibles. La comparaison
du NZD avec la référence du formateur datée du 14 septembre donne environ P20 = −53 %
et P80 = −36 %, comme sur l'image. Ce sont les percentiles du script personnel
reproduits sur la courbe historique — **pas** le classement acheteurs/vendeurs 0–100
et **pas** les seuils institutionnels du CME décrits ailleurs dans la formation.

Si les niveaux de référence sont disponibles, les passer explicitement :

```bash
.venv/bin/python cot_dashboard.py \
  --csv rapport.csv \
  --levels-csv niveaux.csv
```

Le CSV des seuils utilise les colonnes `currency,lower_level,upper_level`. Cela remplace
les percentiles pour les devises déclarées dans ce fichier.

`--no-percentile-bands` masque ces percentiles lorsqu'on veut comparer la courbe seule.

Le graphique USD du dashboard utilise le proxy COT pondéré DXY SEK-free décrit ci-dessus.
Le DXY historique CFTC reste exporté pour comparaison, mais n'est pas publié comme
graphique du dossier USD.

## Tests

```bash
.venv/bin/python -m unittest -v test_cot_dashboard.py
```
