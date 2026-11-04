# Guide de Vérification Manuelle des Données Économiques

**Objectif:** Vérifier l'exactitude des données économiques fournies par Gemini avant intégration.

**Contexte:**
- Données de Gemini couvrent la période **2023-11-01 à 2026-10-01** (avec des prévisions)
- Période cible requise: **2022-10-01 à 2025-09-01** (36 mois) + **2022-10-01 à 2025-07-01** (12 trimestres)
- ⚠️ **Données futures supprimées:** Toutes les valeurs après 2025-09-01 (mensuel) et 2025-07-01 (trimestriel)
- ❌ **Données manquantes:** 2022-10-01 à 2023-10-01 (13 mois + 4 trimestres)

## 🔍 Instructions de Vérification

### 1. **Accéder à Investing.com**
- Allez sur [https://www.investing.com](https://www.investing.com)
- Recherchez l'indicateur économique spécifique
- Sélectionnez l'onglet **"Historical Data"**
- **IMPORTANT:** Utilisez **UNIQUEMENT** la colonne **"Actual"**, jamais **"Forecast"** ou **"Previous"**

### 2. **Vérifier les valeurs**
Pour chaque indicateur ci-dessous:
- Comparez les valeurs de Gemini (dans les fichiers `history_cleaned.csv`) avec Investing.com
- Notez les divergences dans la section **"Résultats de Vérification"** ci-dessous

---

## 📋 Liste de Vérification par Devise

### EUR (Eurozone)

#### Taux Directeur (ECB Deposit Facility Rate)
- **URL Investing.com:** [ECB Deposit Facility Rate](https://www.investing.com/central-banks/ecb)
- **Valeurs à vérifier:**
  - 2024-01-01: 4.00 ✅ (vérifié via finance_search)
  - 2024-06-01: 3.75 ✅ (vérifié)
  - 2024-09-01: 3.50 ✅ (vérifié)
  - 2024-12-01: 3.00 ✅ (vérifié)
  - 2025-03-01: 2.50 (à vérifier)
  - 2025-06-01: 2.00 (à vérifier)

#### HICP Headline YoY
- **URL:** [Eurozone Inflation Rate](https://www.investing.com/economic-calendar/euro-zone-inflation-rate)
- **Valeurs à vérifier (2024):**
  - 2024-01: 2.8
  - 2024-06: 2.5
  - 2024-09: 1.7
  - 2024-12: 2.0

#### HICP Core YoY
- **URL:** [Eurozone Core Inflation Rate](https://www.investing.com/economic-calendar/euro-zone-core-inflation-rate)
- **Valeurs à vérifier (2024):**
  - 2024-01: 3.3
  - 2024-06: 2.9
  - 2024-09: 2.7
  - 2024-12: 2.7

#### PPI YoY
- **URL:** [Eurozone Producer Prices](https://www.investing.com/economic-calendar/euro-zone-producer-prices)

#### Unemployment Rate
- **URL:** [Eurozone Unemployment Rate](https://www.investing.com/economic-calendar/euro-zone-unemployment-rate)

#### Real GDP Growth (QoQ)
- **URL:** [Eurozone GDP Growth](https://www.investing.com/economic-calendar/euro-zone-gdp-growth)

#### Manufacturing PMI
- **URL:** [Eurozone Manufacturing PMI](https://www.investing.com/economic-calendar/euro-zone-manufacturing-pmi)

#### Services PMI
- **URL:** [Eurozone Services PMI](https://www.investing.com/economic-calendar/euro-zone-services-pmi)

---

### GBP (United Kingdom)

#### Taux Directeur (BoE Bank Rate)
- **URL:** [UK Interest Rate](https://www.investing.com/central-banks/uk)
- **Valeurs à vérifier:**
  - 2024-01-01: 5.25
  - 2024-06-01: 5.00
  - 2024-08-01: 5.25
  - 2024-12-01: 5.00

#### CPI Headline YoY
- **URL:** [UK Inflation Rate](https://www.investing.com/economic-calendar/uk-inflation-rate)

#### CPI Core YoY
- **URL:** [UK Core Inflation Rate](https://www.investing.com/economic-calendar/uk-core-inflation-rate)

#### PPI YoY
- **URL:** [UK Producer Prices](https://www.investing.com/economic-calendar/uk-producer-prices)

#### Manufacturing PMI
- **URL:** [UK Manufacturing PMI](https://www.investing.com/economic-calendar/uk-manufacturing-pmi)

#### Services PMI
- **URL:** [UK Services PMI](https://www.investing.com/economic-calendar/uk-services-pmi)

---

### CHF (Switzerland)

#### Taux Directeur (SNB Policy Rate)
- **URL:** [Switzerland Interest Rate](https://www.investing.com/central-banks/switzerland)
- **⚠️ PROBLÈME IDENTIFIÉ:**
  - **Gemini:** 1.75 (2023-11), 1.50 (2024-03), 1.25 (2024-06), 1.00 (2024-09), 0.50 (2024-12), 0.25 (2025-03), 0.00 (2025-06)
  - **Valeurs suspectes:** La SNB n'a pas baissé les taux aussi agressivement. Vérifier sur Investing.com

#### CPI Headline YoY
- **URL:** [Switzerland Inflation Rate](https://www.investing.com/economic-calendar/swiss-inflation-rate)
- **⚠️ PROBLÈME IDENTIFIÉ:**
  - **Gemini:** 1.4-1.7 (2023-11 à 2024-01), puis chute à 0.6-1.4
  - **Réalité attendue:** L'inflation suisse était plutôt autour de 1.5-2.5% en 2024, pas 0.6%

#### CPI Core YoY
- **URL:** [Switzerland Core Inflation Rate](https://www.investing.com/economic-calendar/swiss-core-inflation-rate)

#### PPI YoY
- **URL:** [Switzerland Producer Prices](https://www.investing.com/economic-calendar/swiss-producer-prices)

---

### CAD (Canada)

#### Taux Directeur (BoC Overnight Rate)
- **URL:** [Canada Interest Rate](https://www.investing.com/central-banks/canada)

#### CPI Common
- **URL:** [Canada CPI Common](https://www.investing.com/economic-calendar/canada-cpi-common)

#### CPI Trimmed
- **URL:** [Canada CPI Trimmed](https://www.investing.com/economic-calendar/canada-cpi-trimmed)

#### CPI Median
- **URL:** [Canada CPI Median](https://www.investing.com/economic-calendar/canada-cpi-median)

---

### AUD (Australia)

#### Taux Directeur (RBA Cash Rate)
- **URL:** [Australia Interest Rate](https://www.investing.com/central-banks/australia)

#### CPI Trimmed Mean
- **URL:** [Australia CPI Trimmed Mean](https://www.investing.com/economic-calendar/australia-cpi-trimmed-mean)

#### CPI Weighted Median
- **URL:** [Australia CPI Weighted Median](https://www.investing.com/economic-calendar/australia-cpi-weighted-median)

---

### NZD (New Zealand)

#### Taux Directeur (RBNZ OCR)
- **URL:** [New Zealand Interest Rate](https://www.investing.com/central-banks/new-zealand)

---

### JPY (Japan)

#### Taux Directeur (BoJ Policy Rate)
- **URL:** [Japan Interest Rate](https://www.investing.com/central-banks/japan)

---

### CNY (China)

#### Taux Directeur (PBOC 7-Day Reverse Repo)
- **URL:** [China Interest Rate](https://www.investing.com/central-banks/china)

#### CPI Headline YoY
- **URL:** [China Inflation Rate](https://www.investing.com/economic-calendar/china-inflation-rate)

#### Caixin Manufacturing PMI
- **URL:** [Caixin China Manufacturing PMI](https://www.investing.com/economic-calendar/caixin-china-manufacturing-pmi)

#### Caixin Services PMI
- **URL:** [Caixin China Services PMI](https://www.investing.com/economic-calendar/caixin-china-services-pmi)

---

## 📊 Résultats de Vérification

### ✅ Vérifié et Approuvé

| Devise | Série | Période | Résultat | Notes |
|--------|-------|---------|----------|-------|
| EUR | policy_rate | 2024-01 à 2024-12 | ✅ Exact | Vérifié via finance_search API |

### ⚠️ À Vérifier (Priorité Haute)

| Devise | Série | Période | Valeur Gemini | Attendu | Notes |
|--------|-------|---------|----------------|---------|-------|
| CHF | policy_rate | 2024-09 | 1.00 | ~1.50 ? | Vérifier la décision de la SNB en septembre 2024 |
| CHF | policy_rate | 2024-12 | 0.50 | ~1.25 ? | La SNB n'a probablement pas baissé aussi vite |
| CHF | cpi_headline_yoy | 2024-01 | 1.3 | ~1.7-2.0 | Valeur trop basse |
| CHF | cpi_headline_yoy | 2024-10 | 0.6 | ~1.5-2.0 | Valeur trop basse |

### ❌ Erreurs Confirmées

| Devise | Série | Période | Valeur Gemini | Valeur Correcte | Source |
|--------|-------|---------|----------------|-----------------|--------|
| (À compléter après vérification) | | | | | |

---

## 📝 Notes de Vérification

1. **Données futures:** Toutes les valeurs après **2025-09-01** (mensuel) et **2025-07-01** (trimestriel) ont été **supprimées** car considérées comme des prévisions.

2. **Période manquante:** Les données de **2022-10-01 à 2023-10-01** sont **absentes** des fichiers Gemini. Ces données doivent être ajoutées manuellement à partir de sources officielles.

3. **Précision des dates:** Les dates sont au format **YYYY-MM-01** pour les données mensuelles et **YYYY-01-01, YYYY-04-01, etc.** pour les données trimestrielles.

4. **Sources prioritaires:**
   - **1ère priorité:** Investing.com (valeurs "Actual")
   - **2ème priorité:** Site officiel de l'organisme (ECB, Eurostat, BoE, etc.)
   - **3ème priorité:** S&P Global pour les PMI

---

## 🎯 Prochaines Étapes

1. **Vérifiez d'abord les données CHF** (les plus suspectes)
2. **Vérifiez les taux directeurs** pour toutes les devises (le plus facile)
3. **Complétez les données manquantes** (2022-10 à 2023-10)
4. **Exécutez le script de validation:**
   ```bash
   cd /home/david/macro_journal/tools/economic_data_importer
   cargo run -- validate --data-dir ../../economic_data
   ```

---

**Date de création:** 2026-11-04  
**Responsable:** [Votre nom]  
**Statut:** En cours de vérification ⚠️
