#!/usr/bin/env python3
"""
Script pour télécharger les données économiques suisses (CHF) depuis les sources officielles.

Sources:
- SNB (Swiss National Bank) : Taux directeur
- FSO (Federal Statistical Office) : CPI, PPI, Chômage
- SECO (State Secretariat for Economic Affairs) : PIB réel
- S&P Global : PMI Manufacturier & Services

Période: 36 derniers mois (octobre 2022 - septembre 2026) + 12 derniers trimestres
"""

import requests
import csv
import os
from datetime import datetime, timedelta
from pathlib import Path
import pandas as pd
import io

# Configuration
DATA_DIR = Path(__file__).parent / "CHF"
DATA_DIR.mkdir(parents=True, exist_ok=True)
OUTPUT_FILE = DATA_DIR / "history.csv"

# User-Agent pour éviter le blocage
HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
}


def download_snb_policy_rate() -> pd.DataFrame:
    """
    Télécharger le taux directeur de la SNB (SNB Policy Rate).
    Source: https://data.snb.ch/en/topics/zireb#!/cube/monpol
    """
    print("🔄 Téléchargement du taux directeur SNB...")
    
    # La SNB publie les données ici
    url = "https://www.snb.ch/content/dam/snb/en/ifor/statpub/4zireb/snb_zireb_monpol.csv"
    
    try:
        response = requests.get(url, headers=HEADERS, timeout=10)
        if response.status_code == 200:
            # Parser le CSV de la SNB
            df = pd.read_csv(io.StringIO(response.text), sep=";")
            # Filtrer par "SNB policy rate"
            policy_rate = df[df["Indicator"] == "SNB policy rate"]
            
            # Formater les dates et valeurs
            policy_rate = policy_rate[["Date", "Value"]].copy()
            policy_rate.columns = ["date", "value"]
            policy_rate["series_id"] = "policy_rate"
            policy_rate["value_secondary"] = ""
            
            # Convertir les dates en YYYY-MM-01
            policy_rate["date"] = pd.to_datetime(policy_rate["date"])
            policy_rate["date"] = policy_rate["date"].dt.to_period("M").dt.to_timestamp().dt.strftime("%Y-%m-%d")
            
            # Filtrer la période : oct 2022 - sept 2026
            start_date = "2022-10-01"
            end_date = "2026-09-01"
            mask = (policy_rate["date"] >= start_date) & (policy_rate["date"] <= end_date)
            policy_rate = policy_rate[mask]
            
            print(f"✅ Taux directeur: {len(policy_rate)} observations")
            return policy_rate[["series_id", "date", "value", "value_secondary"]]
        else:
            print(f"⚠️  Erreur SNB: {response.status_code}")
    except Exception as e:
        print(f"❌ Erreur SNB: {e}")
    
    return pd.DataFrame()


def download_fso_cpi() -> pd.DataFrame:
    """
    Télécharger le CPI suisse (globale et core) depuis le FSO.
    Source: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html
    """
    print("🔄 Téléchargement du CPI FSO...")
    
    # Le FSO a des données CSV accessibles
    # CPI global: https://www.bfs.admin.ch/content/dam/bfs/bfsclient/web/de/statistik/preise/konsumentenpreise/konsumentenpreiseindex.csv
    # Mais essayons une approche différente
    
    # Utiliser l'API du FSO (si disponible)
    # Sinon, utiliser les données de la SNB qui publie aussi le CPI
    
    # Pour l'instant, retourner un DataFrame vide (à compléter manuellement)
    print("⚠️  FSO CPI: Téléchargement manuel recommandé")
    print("   URL: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html")
    return pd.DataFrame()


def download_fso_ppi() -> pd.DataFrame:
    """
    Télécharger le PPI suisse depuis le FSO.
    Source: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html
    """
    print("🔄 Téléchargement du PPI FSO...")
    print("⚠️  FSO PPI: Téléchargement manuel recommandé")
    print("   URL: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html")
    return pd.DataFrame()


def download_seco_unemployment() -> pd.DataFrame:
    """
    Télécharger le taux de chômage suisse depuis le SECO.
    Source: https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html
    """
    print("🔄 Téléchargement du chômage SECO...")
    print("⚠️  SECO Chômage: Téléchargement manuel recommandé")
    print("   URL: https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html")
    return pd.DataFrame()


def download_seco_gdp() -> pd.DataFrame:
    """
    Télécharger le PIB réel suisse depuis le SECO.
    Source: https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html
    """
    print("🔄 Téléchargement du PIB SECO...")
    print("⚠️  SECO PIB: Téléchargement manuel recommandé")
    print("   URL: https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html")
    return pd.DataFrame()


def download_sp_global_pmi() -> pd.DataFrame:
    """
    Télécharger les PMI suisse (Credit Suisse) depuis S&P Global.
    Source: https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx
    """
    print("🔄 Téléchargement des PMI S&P Global...")
    print("⚠️  PMI Suisse: Téléchargement manuel recommandé")
    print("   URL: https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx")
    print("   Sélectionner: Switzerland (Credit Suisse)")
    return pd.DataFrame()


def main():
    """Télécharger toutes les séries CHF."""
    print("=" * 60)
    print("🇨🇭 Téléchargement des données économiques CHF")
    print("=" * 60)
    
    all_data = []
    
    # Télécharger chaque série
    policy_rate = download_snb_policy_rate()
    if not policy_rate.empty:
        all_data.append(policy_rate)
    
    # Pour les autres séries, afficher les instructions
    cpi = download_fso_cpi()
    ppi = download_fso_ppi()
    unemployment = download_seco_unemployment()
    gdp = download_seco_gdp()
    pmi = download_sp_global_pmi()
    
    print("\n" + "=" * 60)
    print("⚠️  INSTRUCTIONS POUR LES DONNÉES MANQUANTES")
    print("=" * 60)
    print("""
Les données suivantes nécessitent un téléchargement manuel:

1. CPI (globale et core):
   - Source: FSO (Federal Statistical Office)
   - URL: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/consumer-prices.html
   - series_id: cpi_headline_yoy, cpi_core_yoy

2. PPI (YoY):
   - Source: FSO
   - URL: https://www.bfs.admin.ch/bfs/en/home/statistics/prices/producer-import-prices.html
   - series_id: ppi_yoy

3. Chômage:
   - Source: SECO
   - URL: https://www.seco.admin.ch/seco/en/home/Arbeitsmarkt/Arbeitslosigkeit.html
   - series_id: unemployment_rate

4. PIB réel:
   - Source: SECO
   - URL: https://www.seco.admin.ch/seco/en/home/wirtschaftswachstum.brcmsa.html
   - series_id: real_gdp_growth
   - Dates: YYYY-01-01, YYYY-04-01, YYYY-07-01, YYYY-10-01

5. PMI Manufacturier & Services:
   - Source: S&P Global (Credit Suisse)
   - URL: https://www.pmi.spglobal.com/Public/SeriesData/SeriesData.aspx
   - series_id: sp_global_manufacturing_pmi, sp_global_services_pmi

FORMATEZ LES DONNÉES COMME SUIT:
- series_id,date,value,value_secondary
- Dates mensuelles: YYYY-MM-01
- Dates trimestrielles: YYYY-01-01, YYYY-04-01, YYYY-07-01, YYYY-10-01
- Valeurs: nombre avec point décimal (ex: 2.8)
""")
    
    # Si on a des données, les sauvegarder
    if all_data:
        combined = pd.concat(all_data, ignore_index=True)
        # Sauvegarder dans le fichier existant (ajouter à la fin)
        if OUTPUT_FILE.exists():
            existing = pd.read_csv(OUTPUT_FILE)
            # Éviter les doublons
            combined = pd.concat([existing, combined]).drop_duplicates()
        combined.to_csv(OUTPUT_FILE, index=False)
        print(f"\n✅ Sauvegardé {len(combined)} observations dans {OUTPUT_FILE}")
    else:
        print("\n⚠️  Aucune donnée téléchargée automatiquement.")
        print("   Veuillez télécharger manuellement depuis les URLs ci-dessus.")
    
    print("\n" + "=" * 60)


if __name__ == "__main__":
    main()
