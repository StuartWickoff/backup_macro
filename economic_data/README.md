# Données économiques USD

Le tableau USD utilise un historique local unique : `usd/history.csv`. L'application
ne contacte aucune source économique au démarrage et n'a besoin ni d'une clé API ni
d'un service tiers pour afficher les séries. Les noms et liens des organismes sont
fournis à titre de provenance uniquement.

## Utilisation

Dans le dossier USD, chaque indicateur est une entrée compacte. Ouvrez-en une pour :

- choisir une période d'un, deux ou trois ans;
- basculer entre courbe et histogramme;
- consulter la dernière valeur et la série historique;
- ajouter une nouvelle observation. Une date existante est remplacée, ce qui permet
  de saisir une révision.

La fiche écrit directement dans le CSV local. La date représente la période de
l'observation (et pas nécessairement le jour de publication). Les valeurs doivent
utiliser un point comme séparateur décimal. Le CSV conserve au maximum trois ans par
indicateur; les observations plus anciennes sont retirées quand une nouvelle donnée
est enregistrée. Une saisie au-delà de cette fenêtre est refusée. Les données restent
sur cet appareil.

## Format CSV

Le fichier comporte les colonnes `series_id,date,value,value_secondary`. Les dates
sont au format ISO `YYYY-MM-DD`; la valeur secondaire est utilisée pour la fourchette
Fed et les deux mesures PCE.

| `series_id` | Fréquence | Valeurs |
|---|---|---|
| `fed_target_range` | Mensuelle | Borne basse et haute de la fourchette Fed |
| `pce_inflation` | Mensuelle | PCE global et sous-jacent, variation sur 12 mois |
| `unemployment_rate` | Mensuelle | Taux de chômage désaisonnalisé |
| `nonfarm_payrolls` | Mensuelle | Niveau de l'emploi non agricole, en milliers |
| `real_gdp_growth` | Trimestrielle | Croissance trimestrielle annualisée du PIB réel |
| `ism_manufacturing_pmi` | Mensuelle | Indice PMI ISM, entre 0 et 100 |

L'historique fourni a été importé une fois depuis FRED puis enregistré localement.
Les transformations PCE et PIB sont déjà matérialisées dans le CSV : aucune
transformation ni connexion externe n'est requise pour consulter les séries. Le PMI
est prêt à être rempli manuellement; les autres valeurs peuvent être mises à jour
depuis l'interface.
