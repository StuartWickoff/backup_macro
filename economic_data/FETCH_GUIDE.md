# Guide de Téléchargement des Données Économiques

> **Objectif** : Obtenir les données économiques historiques pour les 8 devises (EUR, GBP, JPY, CHF, CAD, AUD, NZD, CNY) avec **36 mois + 12 trimestres** de couverture.

> **⚠️ IMPORTANT** : Ne pas inventer de valeurs. Utiliser uniquement les **sources officielles** ou **Investing.com** (si accès manuel possible).

---

## 🎯 **Méthode Recommandée**

### **1. Téléchargement Manuel (Recommandé)**
Pour chaque indicateur, suivre ces étapes :

1. **Aller sur le site de la source officielle** (voir tableau ci-dessous)
2. **Trouver la série temporelle** pour l'indicateur
3. **Sélectionner la période** : Octobre 2022 à Septembre 2026 (36 mois)
4. **Télécharger en CSV**
5. **Formater les données** selon le format requis
6. **Sauvegarder dans** `economic_data/<currency>/history.csv`

---

## 📊 **Tableau des Sources par Devise**

### **EUR (Zone Euro)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** (Deposit Facility Rate) | ECB | [https://www.ecb.europa.eu/stats/policy_and_exchange_rates/key_ecb_interest_rates/html/index.en.html](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/key_ecb_interest_rates/html/index.en.html) | Télécharger "Historical data" → CSV |
| **HICP (globale)** | Eurostat | [https://ec.europa.eu/eurostat/web/products-datasets/-/prc_hicp_manr](https://ec.europa.eu/eurostat/web/products-datasets/-/prc_hicp_manr) | Filtrer : EA19/EA20, YoY, mensuel |
| **HICP (core)** | Eurostat | [https://ec.europa.eu/eurostat/web/products-datasets/-/prc_hicp_ctm](https://ec.europa.eu/eurostat/web/products-datasets/-/prc_hicp_ctm) | Exclure énergie et nourriture |
| **PPI** | Eurostat | [https://ec.europa.eu/eurostat/web/products-datasets/-/prc_ppim](https://ec.europa.eu/eurostat/web/products-datasets/-/prc_ppim) | Indice YoY |
| **Chômage** | Eurostat | [https://ec.europa.eu/eurostat/web/products-datasets/-/une_rt_m](https://ec.europa.eu/eurostat/web/products-datasets/-/une_rt_m) | Taux désaisonnalisé |
| **PIB réel** | Eurostat | [https://ec.europa.eu/eurostat/web/products-datasets/-/namq_gdp_k](https://ec.europa.eu/eurostat/web/products-datasets/-/namq_gdp_k) | QoQ, volume |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Eurozone |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Eurozone |

---

### **GBP (Royaume-Uni)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** (Bank Rate) | BoE | [https://www.bankofengland.co.uk/markets/official-bank-rate](https://www.bankofengland.co.uk/markets/official-bank-rate) | Télécharger CSV |
| **CPI (globale)** | ONS | [https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceinflation](https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceinflation) | Table CPIH01 |
| **CPI (core)** | ONS | [https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceinflation](https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/consumerpriceinflation) | Exclure énergie, nourriture, alcool, tabac |
| **PPI** | ONS | [https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/producerpriceinflation](https://www.ons.gov.uk/economy/inflationandpriceindices/datasets/producerpriceinflation) | Output PPI YoY |
| **Chômage** | ONS | [https://www.ons.gov.uk/employmentandlabourmarket/peopleinwork/employmentandemployeetypes/datasets/unemploymentbyageandduration](https://www.ons.gov.uk/employmentandlabourmarket/peopleinwork/employmentandemployeetypes/datasets/unemploymentbyageandduration) | Taux ILO désaisonnalisé |
| **PIB réel** | ONS | [https://www.ons.gov.uk/economy/grossdomesticproductgdp/datasets/grossdomesticproductpreliminaryestimate](https://www.ons.gov.uk/economy/grossdomesticproductgdp/datasets/grossdomesticproductpreliminaryestimate) | QoQ |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner UK |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner UK |

---

### **JPY (Japon)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | BoJ | [https://www.boj.or.jp/en/statistics/boj/other/market/index.htm](https://www.boj.or.jp/en/statistics/boj/other/market/index.htm) | Policy Rate |
| **CPI (globale)** | Statistics Japan | [https://www.stat.go.jp/english/data/cpi/index.html](https://www.stat.go.jp/english/data/cpi/index.html) | CPI All Japan YoY |
| **CPI (core)** | Statistics Japan | [https://www.stat.go.jp/english/data/cpi/index.html](https://www.stat.go.jp/english/data/cpi/index.html) | Exclure frais alimentaires frais |
| **PPI** | BoJ | [https://www.boj.or.jp/en/statistics/price/index.htm](https://www.boj.or.jp/en/statistics/price/index.htm) | CGPI (Corporate Goods Price Index) YoY |
| **Chômage** | Statistics Japan | [https://www.stat.go.jp/english/data/roudou/longtime/index.html](https://www.stat.go.jp/english/data/roudou/longtime/index.html) | Taux désaisonnalisé |
| **PIB réel** | Cabinet Office | [https://www5.cao.go.jp/keizai3/gdp-e.html](https://www5.cao.go.jp/keizai3/gdp-e.html) | QoQ |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Japan (au Jibun Bank) |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Japan (au Jibun Bank) |

---

### **CHF (Suisse)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | SNB | [https://data.snb.ch/en/topics/zireb#!/cube/monpol](https://data.snb.ch/en/topics/zireb#!/cube/monpol) | SNB Policy Rate |
| **CPI (globale)** | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html) | YoY |
| **CPI (core)** | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html) | Exclure produits énergétiques et alimentaires frais |
| **PPI** | FSO | [https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html](https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html) | YoY |
| **Chômage** | SECO | [https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html](https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html) | Taux désaisonnalisé |
| **PIB réel** | SECO | [https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html](https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html) | QoQ |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Switzerland (Credit Suisse) |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Switzerland (Credit Suisse) |

---

### **CAD (Canada)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | BoC | [https://www.bankofcanada.ca/rates/interest-rates/canadian-interest-rates/](https://www.bankofcanada.ca/rates/interest-rates/canadian-interest-rates/) | Overnight Target Rate |
| **CPI Common** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501) | YoY |
| **CPI Trimmed** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501) | YoY |
| **CPI Median** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000501) | YoY |
| **PPI** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000801](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1810000801) | IPPI YoY |
| **Chômage** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1410028001](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=1410028001) | Taux désaisonnalisé |
| **PIB réel** | StatCan | [https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=3610022201](https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=3610022201) | QoQ annualisé |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Canada |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Canada |

---

### **AUD (Australie)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | RBA | [https://www.rba.gov.au/statistics/cash-rate/](https://www.rba.gov.au/statistics/cash-rate/) | Cash Rate Target |
| **CPI Trimmed Mean** | ABS | [https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/consumer-price-index-australia/latest-release](https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/consumer-price-index-australia/latest-release) | YoY |
| **CPI Weighted Median** | ABS | [https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/consumer-price-index-australia/latest-release](https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/consumer-price-index-australia/latest-release) | YoY |
| **PPI** | ABS | [https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/producer-price-indexes-australia/latest-release](https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/producer-price-indexes-australia/latest-release) | YoY |
| **Chômage** | ABS | [https://www.abs.gov.au/statistics/labour/employment-and-unemployment/labour-force-australia/latest-release](https://www.abs.gov.au/statistics/labour/employment-and-unemployment/labour-force-australia/latest-release) | Taux désaisonnalisé |
| **PIB réel** | ABS | [https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product/latest-release](https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product/latest-release) | QoQ |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Australia (Judo Bank) |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner Australia (Judo Bank) |

---

### **NZD (Nouvelle-Zélande)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | RBNZ | [https://www.rbnz.govt.nz/statistics/key-graphs/interest-rates](https://www.rbnz.govt.nz/statistics/key-graphs/interest-rates) | Official Cash Rate (OCR) |
| **CPI (globale)** | Stats NZ | [https://www.stats.govt.nz/indicators/consumer-price-index-cpi](https://www.stats.govt.nz/indicators/consumer-price-index-cpi) | YoY |
| **CPI (core)** | Stats NZ | [https://www.stats.govt.nz/indicators/consumer-price-index-cpi](https://www.stats.govt.nz/indicators/consumer-price-index-cpi) | Exclure nourriture et énergie |
| **PPI** | Stats NZ | [https://www.stats.govt.nz/indicators/producers-price-index](https://www.stats.govt.nz/indicators/producers-price-index) | YoY |
| **Chômage** | Stats NZ | [https://www.stats.govt.nz/indicators/unemployment-rate](https://www.stats.govt.nz/indicators/unemployment-rate) | Taux désaisonnalisé |
| **PIB réel** | Stats NZ | [https://www.stats.govt.nz/indicators/gross-domestic-product-gdp](https://www.stats.govt.nz/indicators/gross-domestic-product-gdp) | QoQ |
| **PMI Manufacturier** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner New Zealand |
| **PMI Services** | S&P Global | [https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx](https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx) | Sélectionner New Zealand |

---

### **CNY (Chine)**

| Indicateur | Source | URL | Commentaires |
|------------|--------|-----|--------------|
| **Taux directeur** | PBOC | [http://www.pbc.gov.cn/en/3688110/index.html](http://www.pbc.gov.cn/en/3688110/index.html) | 7-day Reverse Repo Rate |
| **CPI (globale)** | NBS | [https://data.stats.gov.cn/english/easyquery.htm?cn=C01](https://data.stats.gov.cn/english/easyquery.htm?cn=C01) | YoY |
| **CPI (core)** | NBS | [https://data.stats.gov.cn/english/easyquery.htm?cn=C01](https://data.stats.gov.cn/english/easyquery.htm?cn=C01) | Exclure nourriture et énergie |
| **PPI** | NBS | [https://data.stats.gov.cn/english/easyquery.htm?cn=C01](https://data.stats.gov.cn/english/easyquery.htm?cn=C01) | YoY |
| **Chômage** | NBS | [https://data.stats.gov.cn/english/easyquery.htm?cn=C01](https://data.stats.gov.cn/english/easyquery.htm?cn=C01) | Surveyed Unemployment Rate (Urban) |
| **PIB réel** | NBS | [https://data.stats.gov.cn/english/easyquery.htm?cn=C01](https://data.stats.gov.cn/english/easyquery.htm?cn=C01) | QoQ |
| **PMI Manufacturier** | Caixin | [https://www.caixinglobal.com/](https://www.caixinglobal.com/) | Caixin China Manufacturing PMI |
| **PMI Services** | Caixin | [https://www.caixinglobal.com/](https://www.caixinglobal.com/) | Caixin China Services PMI |

---

## 📥 **Script Python pour Télécharger depuis les APIs**

> **⚠️ ATTENTION** : Certaines APIs nécessitent une clé API (gratuite). Crée un fichier `.env` avec tes clés.

### **1. Créer le fichier `.env`**

```bash
# Dans economic_data/
echo "FRED_API_KEY=ta_cle_fred" > .env
echo "ALPHA_VANTAGE_API_KEY=ta_cle_alpha_vantage" >> .env
```

### **2. Script Python `fetch_data.py`**

```python
#!/usr/bin/env python3
"""
Script pour télécharger les données économiques depuis les APIs publiques.
"""

import os
import requests
import csv
import pandas as pd
from datetime import datetime, timedelta
from pathlib import Path
from dotenv import load_dotenv

# Charger les variables d'environnement
load_dotenv()

FRED_API_KEY = os.getenv("FRED_API_KEY")
ALPHA_VANTAGE_API_KEY = os.getenv("ALPHA_VANTAGE_API_KEY")

# Configuration
DATA_DIR = Path("economic_data")
START_DATE = "2022-10-01"  # 36 mois avant oct 2026
END_DATE = "2026-09-01"   # Dernier mois

# Series IDs par devise (à compléter)
SERIES_MAP = {
    "EUR": {
        "policy_rate": "IR.PK.M.U2.EUR.RT.MIN.50",  # ECB Deposit Facility Rate (FRED)
        "hicp_headline_yoy": "CP0000EZ19M086NEST",    # HICP All Items (FRED)
        "hicp_core_yoy": None,  # À trouver
    },
    # Ajouter les autres devises ici
}

def fetch_fred_series(series_id: str, start_date: str, end_date: str) -> pd.DataFrame:
    """Télécharger une série depuis FRED API."""
    url = f"https://api.stlouisfed.org/fred/series/observations?"
    params = {
        "series_id": series_id,
        "api_key": FRED_API_KEY,
        "file_type": "json",
        "start": start_date,
        "end": end_date,
    }
    
    response = requests.get(url, params=params)
    data = response.json()
    
    if "error_code" in data:
        print(f"Erreur FRED pour {series_id}: {data.get('error_message')}")
        return pd.DataFrame()
    
    # Convertir en DataFrame
    observations = data.get("observations", [])
    if not observations:
        return pd.DataFrame()
    
    df = pd.DataFrame(observations)
    df = df[["date", "value"]]
    df["value"] = pd.to_numeric(df["value"], errors="coerce")
    df = df.dropna()
    
    # Filtrer par date
    df["date"] = pd.to_datetime(df["date"])
    mask = (df["date"] >= start_date) & (df["date"] <= end_date)
    df = df[mask]
    
    # Formater les dates en YYYY-MM-01
    df["date"] = df["date"].dt.to_period("M").dt.to_timestamp().dt.strftime("%Y-%m-%d")
    
    return df

def save_to_csv(df: pd.DataFrame, currency: str, series_id: str) -> None:
    """Sauvegarder les données dans le bon format."""
    if df.empty:
        print(f"Aucune donnée pour {currency} - {series_id}")
        return
    
    # Ajouter series_id
    df["series_id"] = series_id
    df = df[["series_id", "date", "value"]]
    df["value_secondary"] = ""  # Vide pour l'instant
    
    # Chemin du fichier
    currency_dir = DATA_DIR / currency
    currency_dir.mkdir(parents=True, exist_ok=True)
    csv_path = currency_dir / "history.csv"
    
    # Sauvegarder (ajouter à la fin du fichier existant)
    if csv_path.exists():
        # Lire le fichier existant
        existing = pd.read_csv(csv_path)
        # Fusionner (supprimer les doublons)
        combined = pd.concat([existing, df]).drop_duplicates()
        combined.to_csv(csv_path, index=False)
    else:
        # Créer un nouveau fichier avec l'en-tête
        df.to_csv(csv_path, index=False)
    
    print(f"Sauvegardé {len(df)} observations pour {currency} - {series_id}")

def main():
    """Télécharger toutes les séries."""
    print("Début du téléchargement des données économiques...")
    
    for currency, series_map in SERIES_MAP.items():
        print(f"\nTraitement de {currency}...")
        for series_id, fred_id in series_map.items():
            if fred_id is None:
                print(f"  {series_id}: ID FRED non disponible")
                continue
            
            print(f"  Téléchargement de {series_id} (FRED: {fred_id})...")
            df = fetch_fred_series(fred_id, START_DATE, END_DATE)
            if not df.empty:
                save_to_csv(df, currency, series_id)

if __name__ == "__main__":
    main()
```

---

## 📝 **Instructions Pas à Pas**

### **Pour chaque devise** :

1. **Créer le dossier** :
   ```bash
   mkdir -p economic_data/EUR
   ```

2. **Télécharger les données** :
   - Aller sur le site de la source officielle (ex: [ECB](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/key_ecb_interest_rates/html/index.en.html) pour EUR)
   - Trouver la série temporelle
   - Sélectionner la période : **Octobre 2022 à Septembre 2026**
   - Télécharger en **CSV**

3. **Formater les données** :
   - Ouvrir le CSV dans un tableur (Excel, LibreOffice, Google Sheets)
   - Garder uniquement les colonnes : **Date, Valeur**
   - Renommer les colonnes : **series_id, date, value, value_secondary**
   - Ajouter une colonne `series_id` avec le nom de la série (ex: `policy_rate`)
   - Formater les dates : **YYYY-MM-01** (ex: `2024-01-01`)
   - Formater les valeurs : **nombre avec point décimal** (ex: `2.8`)
   - La colonne `value_secondary` peut rester vide

4. **Fusionner les séries** :
   - Pour chaque devise, fusionner tous les CSV des indicateurs dans un seul fichier `history.csv`
   - Supprimer les doublons
   - Trier par `series_id` puis `date`

5. **Sauvegarder** :
   ```bash
   # Exemple pour EUR
   cp formatted_data.csv economic_data/EUR/history.csv
   ```

---

## ✅ **Vérification**

Une fois toutes les données téléchargées :

1. **Vérifier le format** :
   ```bash
   head -5 economic_data/EUR/history.csv
   ```
   Doit afficher :
   ```csv
   series_id,date,value,value_secondary
   policy_rate,2022-10-01,1.25,
   policy_rate,2022-11-01,1.5,
   ...
   ```

2. **Compter les observations** :
   ```bash
   # Pour EUR
   wc -l economic_data/EUR/history.csv
   ```
   Doit afficher environ **8 séries × 36 mois = 288 lignes** (plus l'en-tête).

3. **Valider avec l'outil Rust** :
   ```bash
   cargo run --package economic-data-importer -- validate --data-dir ./economic_data
   ```

---

## 🎯 **Exemple Complet pour EUR**

### **1. Télécharger le taux directeur (ECB)**
- URL : [https://www.ecb.europa.eu/stats/policy_and_exchange_rates/key_ecb_interest_rates/html/index.en.html](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/key_ecb_interest_rates/html/index.en.html)
- Cliquer sur "Deposit facility" → "Historical data" → "CSV"
- Sélectionner : **From: 2022-10-01, To: 2026-09-01**

### **2. Formater le CSV**

**Avant (exemple)** :
```csv
Date,Deposit facility rate
2022-10-27,1.25
2022-11-02,1.5
...
```

**Après** :
```csv
series_id,date,value,value_secondary
policy_rate,2022-10-01,1.25,
policy_rate,2022-11-01,1.5,
...
```

> **⚠️ Note** : Les dates de l'ECB sont les dates d'effet. Utiliser le **1er jour du mois** pour la date (ex: `2022-10-01` au lieu de `2022-10-27`).

### **3. Répéter pour tous les indicateurs**

Faire la même chose pour :
- HICP (inflation globale)
- HICP Core (inflation sous-jacente)
- PPI
- Chômage
- PIB réel (trimestriel → dates : 2023-01-01, 2023-04-01, etc.)
- PMI Manufacturier
- PMI Services

### **4. Fusionner et sauvegarder**

Fusionner tous les CSV dans `economic_data/EUR/history.csv`.

---

## 📌 **Conseils**

1. **Privilégier les sources officielles** : BCE, Eurostat, BoE, etc. sont plus fiables qu'Investing.com.
2. **Vérifier les définitions** : Assurez-vous que la série correspond exactement à ce qui est demandé (ex: HICP pour EUR, pas CPI national).
3. **Respecter les conventions** :
   - **Dates mensuelles** : `YYYY-MM-01`
   - **Dates trimestrielles** : `YYYY-01-01`, `YYYY-04-01`, `YYYY-07-01`, `YYYY-10-01`
   - **Valeurs** : Point comme séparateur décimal
4. **Ne pas inventer** : Si une valeur manque, laissez-la vide. Ne pas estimer.

---

## 🆘 **Problèmes Courants**

### **1. Les dates ne correspondent pas**
- **Problème** : Le site utilise les dates de publication, pas les dates observées.
- **Solution** : Utiliser toujours le **1er jour du mois** pour les données mensuelles.

### **2. Le format des dates est incorrect**
- **Problème** : Le CSV utilise `DD/MM/YYYY` ou `MM-DD-YYYY`.
- **Solution** : Convertir en `YYYY-MM-DD`.

### **3. Les valeurs utilisent la virgule**
- **Problème** : `2,8` au lieu de `2.8`.
- **Solution** : Remplacer les virgules par des points.

### **4. Des valeurs sont manquantes**
- **Problème** : Certaines séries n'ont pas 36 observations.
- **Solution** : Vérifier si la série existe pour toute la période. Si non, signaler dans `CONTROL_SUMMARY.md`.

---

## ✅ **Checklist Final**

- [ ] Toutes les 8 devises ont un fichier `history.csv`
- [ ] Chaque fichier a l'en-tête : `series_id,date,value,value_secondary`
- [ ] Les dates mensuelles sont au format `YYYY-MM-01`
- [ ] Les dates trimestrielles sont au format `YYYY-01-01`, `YYYY-04-01`, etc.
- [ ] Les valeurs utilisent le point comme séparateur décimal
- [ ] Aucune valeur n'a été inventée
- [ ] Chaque devise a 8 séries (7 mensuelles + 1 trimestrielle)
- [ ] Chaque série mensuelle a environ 36 observations
- [ ] La série trimestrielle (PIB) a environ 12 observations
- [ ] L'outil de validation Rust ne signale aucune erreur

---

## 📚 **Ressources Supplémentaires**

- [SOURCES.md](SOURCES.md) - Documentation complète des sources
- [README.md](README.md) - Structure des données
- [../tools/economic_data_importer/README.md](../tools/economic_data_importer/README.md) - Outil d'import Rust
