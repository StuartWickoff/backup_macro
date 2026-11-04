"""
COT Dashboard — Non-Commercial Positioning (% Open Interest)
Devises : AUD, NZD, USD, EUR, GBP, JPY, CHF, CAD
Format  : Net Non-Commercial % OI + lignes percentiles 52 semaines (20e/80e)
"""

import os
import warnings
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import pandas as pd
import cot_reports as cot

warnings.filterwarnings('ignore')

# Création automatique du dossier de sortie
os.makedirs('outputs', exist_ok=True)

# ─────────────────────────────────────────────
# CONFIGURATION & NOMS OFFICIELS CFTC
# ─────────────────────────────────────────────
CURRENCIES = {
    'AUD': 'AUSTRALIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE',
    'NZD': 'NEW ZEALAND DOLLAR - CHICAGO MERCANTILE EXCHANGE',
    'EUR': 'EURO FX - CHICAGO MERCANTILE EXCHANGE',
    'GBP': 'BRITISH POUND STERLING - CHICAGO MERCANTILE EXCHANGE',
    'JPY': 'JAPANESE YEN - CHICAGO MERCANTILE EXCHANGE',
    'CHF': 'SWISS FRANC - CHICAGO MERCANTILE EXCHANGE',
    'CAD': 'CANADIAN DOLLAR - CHICAGO MERCANTILE EXCHANGE',
    'USD': 'U.S. DOLLAR INDEX - ICE FUTURES U.S.',
}

ROLLING_WINDOW = 52   # Semaines
PCT_LOW        = 0.20
PCT_HIGH       = 0.80
START_DATE     = '2021-01-01'
OUTPUT_PATH    = 'outputs/cot_dashboard.png'

# ─────────────────────────────────────────────
# CHARGEMENT DES DONNÉES CFTC
# ─────────────────────────────────────────────
print("Chargement des données CFTC (Legacy Futures)...")
print("Première exécution : peut prendre 1-2 minutes (téléchargement ZIP CFTC)\n")

frames = []
for year in range(2021, 2027):
    try:
        frames.append(cot.cot_year(year=year, cot_report_type='legacy_fut'))
        print(f"  {year} OK")
    except Exception as e:
        print(f"  {year} indisponible ou erreur : {e}")

if not frames:
    print("Erreur critique : Impossible de charger les données CFTC.")
    exit()

df = pd.concat(frames, ignore_index=True)
print(f"\nDonnées chargées : {len(df)} lignes\n")

# ─────────────────────────────────────────────
# TRAITEMENT PAR DEVISE
# ─────────────────────────────────────────────
def process_currency(df, keyword):
    """Extrait et calcule le Net Non-Commercial % OI pour une devise."""
    mask = df['Market and Exchange Names'].str.strip() == keyword
    sub = df[mask].copy()

    if sub.empty:
        print(f"  Aucune donnée trouvée pour : {keyword}")
        return None

    # Parse date & tri
    sub['Date'] = pd.to_datetime(sub['As of Date in Form YYYY-MM-DD'])
    sub = sub.sort_values('Date').drop_duplicates('Date')
    sub = sub[sub['Date'] >= START_DATE]

    # Calcul Net % OI
    sub['Net_Pct_OI'] = (
        (sub['Noncommercial Positions-Long (All)'] - sub['Noncommercial Positions-Short (All)'])
        / sub['Open Interest (All)'] * 100
    )

    # Calcul des percentiles fixes sur les 52 dernières semaines (lignes droites)
    val_20 = sub['Net_Pct_OI'].tail(ROLLING_WINDOW).quantile(PCT_LOW)
    val_80 = sub['Net_Pct_OI'].tail(ROLLING_WINDOW).quantile(PCT_HIGH)

    sub['Pct_20'] = val_20
    sub['Pct_80'] = val_80

    return sub[['Date', 'Net_Pct_OI', 'Pct_20', 'Pct_80']].dropna(subset=['Net_Pct_OI'])

# ─────────────────────────────────────────────
# GENERATION DES GRAPHIQUES
# ─────────────────────────────────────────────
print("Calcul du positionnement...\n")

results = {}
for ticker, keyword in CURRENCIES.items():
    data = process_currency(df, keyword)
    if data is not None and len(data) > 10:
        results[ticker] = data
        last = data.iloc[-1]
        print(f"{ticker:4s} | Net: {last['Net_Pct_OI']:+6.1f}% | "
              f"P20: {last['Pct_20']:+6.1f}% | P80: {last['Pct_80']:+6.1f}% | "
              f"Date: {last['Date'].strftime('%Y-%m-%d')}")

n = len(results)
if n == 0:
    print("Erreur : Aucune donnée à afficher. Vérifiez vos START_DATE et CURRENCIES.")
    exit()

print(f"\nGénération du dashboard ({n} devises)...")

cols = 2
rows = (n + 1) // cols

fig = plt.figure(figsize=(18, rows * 4.5))
fig.patch.set_facecolor('#0d1117')

gs = gridspec.GridSpec(rows, cols, figure=fig, hspace=0.45, wspace=0.3)

for idx, (ticker, data) in enumerate(results.items()):
    ax = fig.add_subplot(gs[idx // cols, idx % cols])
    ax.set_facecolor('#161b22')

    # Courbe principale Net % OI
    ax.plot(data['Date'], data['Net_Pct_OI'],
            color='#4fc3f7', linewidth=1.5, label='Net Non-Commercial (% OI)', zorder=3)

    # Lignes droites des percentiles (52 dernières semaines)
    ax.plot(data['Date'], data['Pct_20'], '--',
            color='#66bb6a', linewidth=1.2, label=f'{int(PCT_LOW*100)}th Percentile (52w)', zorder=2)
    ax.plot(data['Date'], data['Pct_80'], '--',
            color='#ef5350', linewidth=1.2, label=f'{int(PCT_HIGH*100)}th Percentile (52w)', zorder=2)

    # Zone neutre ombragée
    ax.fill_between(data['Date'], data['Pct_20'], data['Pct_80'],
                    alpha=0.08, color='white', zorder=1)

    # Ligne zéro
    ax.axhline(0, color='#555555', linewidth=0.8, linestyle=':', label='Neutral (0)', zorder=1)

    # Signal visuel sur la dernière valeur
    last = data.iloc[-1]
    color_dot = '#ef5350' if last['Net_Pct_OI'] < last['Pct_20'] else (
                '#66bb6a' if last['Net_Pct_OI'] > last['Pct_80'] else '#4fc3f7')
    ax.scatter(last['Date'], last['Net_Pct_OI'], color=color_dot, s=60, zorder=5)

    # Annotation
    ax.annotate(f"{last['Net_Pct_OI']:+.1f}%",
                xy=(last['Date'], last['Net_Pct_OI']),
                xytext=(8, 0), textcoords='offset points',
                color=color_dot, fontsize=8, fontweight='bold')

    # Styling Dark Theme
    ax.set_title(f'{ticker} – COT Non-Commercial (% Open Interest)',
                 color='white', fontsize=11, fontweight='bold', pad=10)
    ax.set_xlabel('Date', color='#888888', fontsize=8)
    ax.set_ylabel('Percentage', color='#888888', fontsize=8)
    ax.tick_params(colors='#888888', labelsize=7)
    for spine in ax.spines.values():
        spine.set_color('#333333')

    ax.legend(loc='upper left', fontsize=7, framealpha=0.3,
              facecolor='#0d1117', labelcolor='white')

# Titres & Méta
fig.suptitle('COT Dashboard — Non-Commercial Positioning (% Open Interest)',
             color='white', fontsize=14, fontweight='bold', y=1.01)

fig.text(0.5, -0.01, 'Source: CFTC Legacy Futures | Calcul: Percentiles 52 dernières semaines',
         ha='center', color='#444444', fontsize=8)

plt.savefig(OUTPUT_PATH, dpi=150, bbox_inches='tight',
            facecolor='#0d1117', edgecolor='none')
plt.close()

print(f"\nDashboard sauvegardé avec succès : {OUTPUT_PATH}")
print("Done.")
