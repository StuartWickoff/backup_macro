# 🇨🇭 Guide pour Télécharger les Données Économiques CHF

> **Devise** : CHF (Franc Suisse)  
> **Période** : 36 derniers mois (octobre 2022 - septembre 2026) + 12 derniers trimestres  
> **Format** : CSV avec en-tête `series_id,date,value,value_secondary`  

---

## 📋 **Liste des Indicateurs à Télécharger**

| # | Indicateur | series_id | Source | URL | Type | Format Date |
|---|------------|-----------|--------|-----|------|-------------|
| 1 | Taux directeur (SNB Policy Rate) | `policy_rate` | SNB | [https://data.snb.ch/en/topics/zireb#!/cube/monpol](https://data.snb.ch/en/topics/zireb#!/cube/monpol) | Mensuel | YYYY-MM-01 |
| 2 | CPI (globale) - YoY | `cpi_headline_yoy` | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html) | Mensuel | YYYY-MM-01 |
| 3 | CPI (core) - YoY | `cpi_core_yoy` | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html) | Mensuel | YYYY-MM-01 |
| 4 | PPI - YoY | `ppi_yoy` | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html) | Mensuel | YYYY-MM-01 |
| 5 | Chômage | `unemployment_rate` | SECO | [https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html](https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html) | Mensuel | YYYY-MM-01 |
| 6 | PIB réel | `real_gdp_growth` | SECO | [https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html](https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html) | Trimestriel | YYYY-01-01 |
| 7 | PMI Manufacturier | `sp_global_manufacturing_pmi` | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Mensuel | YYYY-MM-01 |
| 8 | PMI Services | `sp_global_services_pmi` | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Mensuel | YYYY-MM-01 |

---

## 🎯 **Instructions Pas à Pas pour Chaque Indicateur**

---

### **1️⃣ Taux Directeur (SNB Policy Rate)**

**Source** : Banque Nationale Suisse (SNB)  
**URL** : [https://data.snb.ch/en/topics/zireb#!/cube/monpol](https://data.snb.ch/en/topics/zireb#!/cube/monpol)

**Étapes** :
1. Aller sur [https://data.snb.ch/en/topics/zireb#!/cube/monpol](https://data.snb.ch/en/topics/zireb#!/cube/monpol)
2. Dans le cube "monpol", sélectionner :
   - **Indicator** : "SNB policy rate"
   - **Frequency** : "Monthly"
   - **Start Period** : "2022-10"
   - **End Period** : "2026-09"
3. Cliquer sur **"Export"** → **"CSV"**
4. Télécharger le fichier CSV

**Formatage** :
- Conserver les colonnes : `Date`, `Value`
- Renommer : `series_id`, `date`, `value`, `value_secondary`
- Ajouter `policy_rate` dans la colonne `series_id`
- Convertir les dates en `YYYY-MM-01` (ex: `2024-03-15` → `2024-03-01`)
- Supprimer les lignes vides

**Exemple de sortie** :
```csv
series_id,date,value,value_secondary
policy_rate,2022-10-01,0.25,
policy_rate,2022-11-01,0.5,
policy_rate,2022-12-01,1.0,
...
```

---

### **2️⃣ CPI (Globale et Core)**

**Source** : Office Fédéral de la Statistique (FSO)  
**URL** : [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html)

**Étapes pour CPI Globale (cpi_headline_yoy)** :
1. Aller sur [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html)
2. Chercher : **"Consumer Price Index - Monthly data"**
3. Sélectionner :
   - **Indicator** : "National Consumer Price Index (CPI)"
   - **Measure** : "Year-on-year change in %"
   - **Period** : "2022-10" à "2026-09"
4. Cliquer sur **"Download"** → **"CSV"**
5. Télécharger le fichier

**Étapes pour CPI Core (cpi_core_yoy)** :
1. Même URL
2. Sélectionner :
   - **Indicator** : "Core Consumer Price Index (excl. fresh food and energy)"
   - **Measure** : "Year-on-year change in %"
   - **Period** : "2022-10" à "2026-09"
3. Télécharger en CSV

**Formatage pour les deux séries** :
- Conserver les colonnes : `Period`, `Value`
- Renommer : `series_id`, `date`, `value`, `value_secondary`
- Ajouter `cpi_headline_yoy` ou `cpi_core_yoy` dans `series_id`
- Convertir les périodes en `YYYY-MM-01` (ex: "2024-M03" → "2024-03-01")

---

### **3️⃣ PPI (Indice des Prix à la Production)**

**Source** : FSO  
**URL** : [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html)

**Étapes** :
1. Aller sur l'URL
2. Chercher : **"Producer and Import Price Index"**
3. Sélectionner :
   - **Measure** : "Producer Price Index (PPI) - Total"  
   - **Change** : "Year-on-year"
   - **Period** : "2022-10" à "2026-09"
4. Télécharger en CSV

**Formatage** :
- `series_id`: `ppi_yoy`
- `date`: Convertir en `YYYY-MM-01`

---

### **4️⃣ Chômage**

**Source** : SECO (State Secretariat for Economic Affairs)  
**URL** : [https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html](https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html)

**Étapes** :
1. Aller sur l'URL
2. Chercher : **"Unemployment rate - Seasonally adjusted"**
3. Sélectionner la période : "2022-10" à "2026-09"
4. Télécharger en CSV

**Formatage** :
- `series_id`: `unemployment_rate`
- `date`: `YYYY-MM-01`
- `value`: Pourcentage (ex: 2.1, pas 2.1%)

---

### **5️⃣ PIB Réel**

**Source** : SECO  
**URL** : [https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html](https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html)

**Étapes** :
1. Aller sur l'URL
2. Chercher : **"GDP - Real - Growth rate compared to previous quarter"**
3. Sélectionner :
   - **Adjustment** : "Seasonally adjusted"
   - **Period** : "2023-Q1" à "2026-Q4" (12 trimestres)
4. Télécharger en CSV

**Formatage** :
- `series_id`: `real_gdp_growth`
- `date`: Convertir en **premier jour du trimestre**
  - Q1 → `YYYY-01-01`
  - Q2 → `YYYY-04-01`
  - Q3 → `YYYY-07-01`
  - Q4 → `YYYY-10-01`
- Exemple: "2024-Q1" → "2024-01-01"

---

### **6️⃣ PMI Manufacturier & Services**

**Source** : S&P Global (anciennement Credit Suisse)  
**URL** : [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx)

**Étapes** :
1. Aller sur l'URL
2. Sélectionner :
   - **Country** : "Switzerland"
   - **Survey** : "Manufacturing PMI" ou "Services PMI"
   - **Period** : "2022-10" à "2026-09"
3. Télécharger en CSV

**Formatage** :
- `series_id`: `sp_global_manufacturing_pmi` ou `sp_global_services_pmi`
- `date`: `YYYY-MM-01`
- `value`: Indice (0-100, ex: 48.5)

---

## 📥 **Méthode Alternative : Utiliser Investing.com (Manuel)**

Si tu préfères utiliser **Investing.com** (source autorisée dans le prompt) :

### **Étapes générales** :
1. Aller sur [https://www.investing.com](https://www.investing.com)
2. Chercher l'indicateur (ex: "Switzerland CPI")
3. Cliquer sur **"Historical Data"**
4. Sélectionner la période : **Oct 01, 2022 - Sep 30, 2026**
5. Cliquer sur **"Download"** (icône CSV)
6. Formater selon les règles ci-dessus

### **URLs directes Investing.com** :
| Indicateur | URL |
|------------|-----|
| Taux directeur | [https://www.investing.com/rates-central-banks/swiss-national-bank](https://www.investing.com/rates-central-banks/swiss-national-bank) |
| CPI | [https://www.investing.com/economic-calendar/swiss-cpi](https://www.investing.com/economic-calendar/swiss-cpi) |
| PPI | [https://www.investing.com/economic-calendar/swiss-ppi](https://www.investing.com/economic-calendar/swiss-ppi) |
| Chômage | [https://www.investing.com/economic-calendar/swiss-unemployment-rate](https://www.investing.com/economic-calendar/swiss-unemployment-rate) |
| PIB | [https://www.investing.com/economic-calendar/swiss-gdp](https://www.investing.com/economic-calendar/swiss-gdp) |
| PMI Manufacturier | [https://www.investing.com/indices/swiss-pmi](https://www.investing.com/indices/swiss-pmi) |
| PMI Services | [https://www.investing.com/indices/swiss-services-pmi](https://www.investing.com/indices/swiss-services-pmi) |

> **⚠️ IMPORTANT** : Utiliser uniquement la valeur **"Actual"** (Réel), jamais "Forecast" ou "Previous".

---

## 📝 **Format Final pour `history.csv`**

**En-tête** :
```csv
series_id,date,value,value_secondary
```

**Exemple complet** :
```csv
series_id,date,value,value_secondary
policy_rate,2022-10-01,-0.25,
policy_rate,2022-11-01,0.0,
policy_rate,2022-12-01,0.5,
policy_rate,2023-01-01,1.0,
...
cpi_headline_yoy,2022-10-01,2.8,
cpi_headline_yoy,2022-11-01,2.7,
cpi_headline_yoy,2022-12-01,2.4,
...
cpi_core_yoy,2022-10-01,1.9,
cpi_core_yoy,2022-11-01,1.8,
...
ppi_yoy,2022-10-01,5.8,
...
unemployment_rate,2022-10-01,2.2,
...
real_gdp_growth,2023-01-01,0.3,
real_gdp_growth,2023-04-01,-0.1,
...
sp_global_manufacturing_pmi,2022-10-01,52.4,
sp_global_services_pmi,2022-10-01,54.2,
...
```

---

## ✅ **Checklist de Validation**

Avant de sauvegarder :

- [ ] **8 séries** présentes (policy_rate, cpi_headline_yoy, cpi_core_yoy, ppi_yoy, unemployment_rate, real_gdp_growth, sp_global_manufacturing_pmi, sp_global_services_pmi)
- [ ] **Dates mensuelles** au format `YYYY-MM-01`
- [ ] **Dates trimestrielles** (PIB) au format `YYYY-01-01`, `YYYY-04-01`, `YYYY-07-01`, `YYYY-10-01`
- [ ] **Valeurs** avec point décimal (ex: `2.8`, pas `2,8`)
- [ ] **Aucune valeur inventée** (laisser vide si manquante)
- [ ] **36 observations mensuelles** par série mensuelle
- [ ] **12 observations trimestrielles** pour le PIB
- [ ] **Pas de doublons** (une ligne par mois/trimestre par série)

---

## 🎯 **Commande de Validation**

Une fois le fichier `history.csv` rempli, valider avec :

```bash
cd /home/david/macro_journal
cargo run --package economic-data-importer -- validate --data-dir ./economic_data
```

L'outil vérifiera automatiquement :
- ✅ Format des dates
- ✅ Présence des 8 séries
- ✅ Couverture temporelle (36 mois / 12 trimestres)
- ✅ Valeurs numériques valides

---

## 📚 **Ressources Supplémentaires**

- [SOURCES.md](../SOURCES.md) - Toutes les sources pour toutes les devises
- [FETCH_GUIDE.md](../FETCH_GUIDE.md) - Guide général de téléchargement
- [README.md](../README.md) - Structure et format des données
- [../tools/economic_data_importer/README.md](../tools/economic_data_importer/README.md) - Outil d'import Rust

---

## 🆘 **Aide**

Si tu as des difficultés avec un indicateur spécifique, me dire lequel et je te guiderai pas à pas !

**Exemple** :
> "Je n'arrive pas à trouver le CPI core suisse sur le site du FSO, peux-tu m'aider ?"

---

## 📌 **Notes Importantes**

1. **Ne pas inventer de données** : Si une valeur manque, laisse-la vide ou contacte-moi.
2. **Respecter les définitions** :
   - CPI Core = CPI hors énergie et nourriture
   - PPI = Indice des prix à la production (YoY)
   - PIB = Croissance du PIB réel (QoQ)
3. **Vérifier les dates** : Utiliser toujours le **1er jour du mois** pour les données mensuelles.
4. **Sources prioritaires** : FSO/SECO > Investing.com

---

**BON TÉLÉCHARGEMENT !** 🚀
