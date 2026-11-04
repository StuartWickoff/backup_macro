#!/usr/bin/env python3
"""
Script pour nettoyer et analyser les données économiques de Gemini.

Problèmes identifiés :
1. Données futures (jusqu'en octobre 2026) - à supprimer (garder jusqu'en septembre 2025)
2. Période manquante (octobre 2022 à octobre 2023) - à compléter
3. Vérification des valeurs nécessaires

Période cible : 36 derniers mois (2022-10 à 2025-09) + 12 derniers trimestres (2022-10 à 2025-07)
Date actuelle : 2026-11-04
"""

import csv
import os
from datetime import datetime, timedelta
from collections import defaultdict
import re

# Configuration
RECHERCHES_DIR = "/home/david/macro_journal/economic_data/recherches_gemini"
OUTPUT_DIR = "/home/david/macro_journal/economic_data"
CUTOFF_DATE = "2025-10-01"  # Date limite pour les données mensuelles (36 mois à partir de 2022-10)
Q_CUTOFF_DATE = "2025-07-01"  # Date limite pour les données trimestrielles (12 trimestres)
START_DATE = "2022-10-01"  # Début de la période cible

# Mappage des noms de fichiers vers codes de devise
FILE_TO_CURRENCY = {
    "eurozone_economic_history.md": "EUR",
    "united_kingdom_economic_history.md": "GBP",
    "japan_economic_history.md": "JPY",
    "switzerland_economic_history.md": "CHF",
    "canada_economic_history.md": "CAD",
    "australia_economic_history.md": "AUD",
    "new_zealand_economic_history.md": "NZD",
    "china_economic_history.md": "CNY",
}

# Séries mensuelles
MONTHLY_SERIES = {
    "policy_rate", "ppi_yoy", "unemployment_rate",
    "hicp_headline_yoy", "hicp_core_yoy", 
    "cpi_headline_yoy", "cpi_core_yoy",
    "cpi_common", "cpi_trimmed", "cpi_median",
    "cpi_trimmed_mean", "cpi_weighted_median",
    "sp_global_manufacturing_pmi", "sp_global_services_pmi",
    "caixin_manufacturing_pmi", "caixin_services_pmi",
}

# Séries trimestrielles
QUARTERLY_SERIES = {"real_gdp_growth"}

# Séries attendues par devise
EXPECTED_SERIES = {
    "EUR": ["policy_rate", "hicp_headline_yoy", "hicp_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth", 
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "GBP": ["policy_rate", "cpi_headline_yoy", "cpi_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "JPY": ["policy_rate", "cpi_headline_yoy", "cpi_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "CHF": ["policy_rate", "cpi_headline_yoy", "cpi_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "CAD": ["policy_rate", "cpi_common", "cpi_trimmed", "cpi_median", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "AUD": ["policy_rate", "cpi_trimmed_mean", "cpi_weighted_median", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "NZD": ["policy_rate", "cpi_headline_yoy", "cpi_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi"],
    "CNY": ["policy_rate", "cpi_headline_yoy", "cpi_core_yoy", "ppi_yoy", 
            "unemployment_rate", "real_gdp_growth",
            "caixin_manufacturing_pmi", "caixin_services_pmi"],
}


def parse_date(date_str):
    """Parse une date au format YYYY-MM-DD"""
    try:
        return datetime.strptime(date_str, "%Y-%m-%d")
    except:
        return None


def is_valid_date(date_str):
    """Vérifie si une date est valide et au bon format"""
    date = parse_date(date_str)
    return date is not None


def get_expected_dates(series_id):
    """Génère les dates attendues pour une série"""
    dates = []
    start = datetime.strptime(START_DATE, "%Y-%m-%d")
    
    if series_id in MONTHLY_SERIES:
        # 36 mois à partir d'octobre 2022
        cutoff = datetime.strptime(CUTOFF_DATE, "%Y-%m-%d")
        current = start
        while current <= cutoff:
            dates.append(current.strftime("%Y-%m-%d"))
            # Ajouter un mois
            if current.month == 12:
                current = current.replace(year=current.year + 1, month=1)
            else:
                current = current.replace(month=current.month + 1)
    elif series_id in QUARTERLY_SERIES:
        # 12 trimestres à partir d'octobre 2022
        cutoff = datetime.strptime(Q_CUTOFF_DATE, "%Y-%m-%d")
        current = start
        while current <= cutoff:
            dates.append(current.strftime("%Y-%m-%d"))
            # Ajouter un trimestre
            if current.month == 10:
                current = current.replace(year=current.year + 1, month=1)
            elif current.month == 1:
                current = current.replace(month=4)
            elif current.month == 4:
                current = current.replace(month=7)
            elif current.month == 7:
                current = current.replace(month=10)
    
    return dates


def load_gemini_data(filepath):
    """Charge les données d'un fichier .md de Gemini"""
    data = []
    with open(filepath, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            data.append({
                'series_id': row['series_id'].strip(),
                'date': row['date'].strip(),
                'value': row['value'].strip(),
                'value_secondary': row['value_secondary'].strip()
            })
    return data


def clean_and_analyze():
    """Nettoie et analyse toutes les données"""
    
    # Dictionnaire pour stocker les résultats par devise
    results = {currency: {'data': [], 'missing_dates': defaultdict(list), 
                          'future_dates': defaultdict(list), 
                          'series_present': set()}
               for currency in EXPECTED_SERIES.keys()}
    
    print("=" * 80)
    print("ANALYSE DES DONNÉES GEMINI")
    print("=" * 80)
    print(f"Date de coupure mensuelle: {CUTOFF_DATE}")
    print(f"Date de coupure trimestrielle: {Q_CUTOFF_DATE}")
    print(f"Période cible: {START_DATE} à {CUTOFF_DATE}")
    print()
    
    # Traiter chaque fichier
    for filename, currency in FILE_TO_CURRENCY.items():
        filepath = os.path.join(RECHERCHES_DIR, filename)
        print(f"\n{'='*80}")
        print(f"TRAITEMENT: {currency} ({filename})")
        print(f"{'='*80}")
        
        if not os.path.exists(filepath):
            print(f"❌ Fichier introuvable: {filepath}")
            continue
            
        data = load_gemini_data(filepath)
        print(f"Données chargées: {len(data)} lignes")
        
        # Organiser par série
        series_data = defaultdict(list)
        for record in data:
            series_data[record['series_id']].append(record)
        
        # Analyser chaque série
        for series_id, records in series_data.items():
            results[currency]['series_present'].add(series_id)
            
            # Vérifier les dates futures
            future_count = 0
            valid_records = []
            for record in records:
                date = record['date']
                
                if series_id in MONTHLY_SERIES:
                    if date > CUTOFF_DATE:
                        results[currency]['future_dates'][series_id].append(date)
                        future_count += 1
                    elif date >= START_DATE:
                        valid_records.append(record)
                elif series_id in QUARTERLY_SERIES:
                    if date > Q_CUTOFF_DATE:
                        results[currency]['future_dates'][series_id].append(date)
                        future_count += 1
                    elif date >= START_DATE:
                        valid_records.append(record)
            
            # Identifier les dates manquantes
            expected_dates = get_expected_dates(series_id)
            actual_dates = {r['date'] for r in valid_records}
            missing = [d for d in expected_dates if d not in actual_dates]
            results[currency]['missing_dates'][series_id] = missing
            
            # Stocker les données valides
            results[currency]['data'].extend(valid_records)
            
            print(f"  {series_id}:")
            print(f"    ✅ Enregistrements valides: {len(valid_records)}")
            print(f"    ⚠️  Dates futures supprimées: {future_count}")
            print(f"    ❌ Dates manquantes: {len(missing)}")
            if missing:
                print(f"       {missing[:5]}{'...' if len(missing) > 5 else ''}")
        
        # Vérifier les séries manquantes
        expected = set(EXPECTED_SERIES[currency])
        present = results[currency]['series_present']
        missing_series = expected - present
        if missing_series:
            print(f"\n❌ Séries manquantes pour {currency}: {missing_series}")
        else:
            print(f"\n✅ Toutes les séries attendues sont présentes pour {currency}")
    
    # Générer le rapport complet
    print("\n" + "=" * 80)
    print("RAPPORT D'ANALYSE COMPLET")
    print("=" * 80)
    
    # Résumé par devise
    for currency in EXPECTED_SERIES.keys():
        print(f"\n{currency}:")
        print(f"  Données valides: {len(results[currency]['data'])}")
        print(f"  Séries présentes: {len(results[currency]['series_present'])}/{len(EXPECTED_SERIES[currency])}")
        
        total_missing = sum(len(dates) for dates in results[currency]['missing_dates'].values())
        print(f"  Dates manquantes: {total_missing}")
        
        total_future = sum(len(dates) for dates in results[currency]['future_dates'].values())
        print(f"  Dates futures: {total_future}")
    
    # Écrire les fichiers CSV nettoyés
    print("\n" + "=" * 80)
    print("GÉNÉRATION DES FICHIERS CSV NETTOYÉS")
    print("=" * 80)
    
    for currency in EXPECTED_SERIES.keys():
        if not results[currency]['data']:
            continue
        
        # Trier les données par series_id puis par date
        sorted_data = sorted(results[currency]['data'], 
                            key=lambda x: (x['series_id'], x['date']))
        
        output_file = os.path.join(OUTPUT_DIR, currency, "history_cleaned.csv")
        
        with open(output_file, 'w', newline='', encoding='utf-8') as f:
            writer = csv.DictWriter(f, fieldnames=['series_id', 'date', 'value', 'value_secondary'])
            writer.writeheader()
            for record in sorted_data:
                writer.writerow(record)
        
        print(f"✅ {currency}: {len(sorted_data)} enregistrements écrits dans {output_file}")
    
    # Générer le rapport de vérification
    print("\n" + "=" * 80)
    print("GÉNÉRATION DU RAPPORT DE VÉRIFICATION")
    print("=" * 80)
    
    report_file = os.path.join(OUTPUT_DIR, "GEMINI_VERIFICATION_REPORT.md")
    with open(report_file, 'w', encoding='utf-8') as f:
        f.write("# Rapport de Vérification des Données Gemini\n\n")
        f.write(f"**Date de génération:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write(f"**Période cible:** {START_DATE} à {CUTOFF_DATE} (mensuel) / {Q_CUTOFF_DATE} (trimestriel)\n\n")
        
        f.write("## Résumé Global\n\n")
        f.write("| Devise | Données valides | Séries complètes | Dates futures supprimées | Dates manquantes |\n")
        f.write("|--------|----------------|-------------------|--------------------------|-----------------|\n")
        
        for currency in EXPECTED_SERIES.keys():
            valid_count = len(results[currency]['data'])
            series_count = len(results[currency]['series_present'])
            total_expected = len(EXPECTED_SERIES[currency])
            future_count = sum(len(dates) for dates in results[currency]['future_dates'].values())
            missing_count = sum(len(dates) for dates in results[currency]['missing_dates'].values())
            
            f.write(f"| {currency} | {valid_count} | {series_count}/{total_expected} | {future_count} | {missing_count} |\n")
        
        f.write("\n## Détails par Devise\n\n")
        
        for currency in EXPECTED_SERIES.keys():
            f.write(f"### {currency}\n\n")
            f.write("**Séries présentes:** ")
            f.write(", ".join(sorted(results[currency]['series_present'])) + "\n\n")
            
            if results[currency]['future_dates']:
                f.write("**Dates futures supprimées:**\n\n")
                for series_id, dates in results[currency]['future_dates'].items():
                    if dates:
                        f.write(f"- {series_id}: {len(dates)} dates ({', '.join(dates[:5])}{'...' if len(dates) > 5 else ''})\n")
                f.write("\n")
            
            if results[currency]['missing_dates']:
                f.write("**Dates manquantes:**\n\n")
                for series_id, dates in results[currency]['missing_dates'].items():
                    if dates:
                        f.write(f"- {series_id}: {len(dates)} dates ({', '.join(dates[:5])}{'...' if len(dates) > 5 else ''})\n")
                f.write("\n")
    
    print(f"✅ Rapport écrit dans {report_file}")
    
    return results


if __name__ == "__main__":
    results = clean_and_analyze()
    
    print("\n" + "=" * 80)
    print("ANALYSE TERMINÉE")
    print("=" * 80)
    print("Fichiers générés:")
    print("- Fichiers CSV nettoyés dans economic_data/<currency>/history_cleaned.csv")
    print("- Rapport de vérification: economic_data/GEMINI_VERIFICATION_REPORT.md")
