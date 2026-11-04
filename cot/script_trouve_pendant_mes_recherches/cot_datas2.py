import cot_reports as cot
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import warnings
import os

os.makedirs('outputs', exist_ok=True)
warnings.filterwarnings('ignore')

CURRENCIES = {
    'AUD': 'AUSTRALIAN DOLLAR',
    'NZD': 'NEW ZEALAND DOLLAR|NZ DOLLAR',
    'EUR': 'EURO FX',
    'GBP': 'BRITISH POUND',
    'JPY': 'JAPANESE YEN',
    'CHF': 'SWISS FRANC',
    'CAD': 'CANADIAN DOLLAR',
}

ROLLING_WINDOW = 52
PCT_LOW = 0.20
PCT_HIGH = 0.80
START_DATE = '2021-01-01'

# Chargement
print("Chargement données CFTC...")
frames = []
for year in range(2021, 2027):
    try:
        frames.append(cot.cot_year(year=year, cot_report_type='legacy_fut'))
        print(f" {year} OK")
    except Exception as e:
        print(f" {year} erreur: {e}")

df = pd.concat(frames, ignore_index=True)
print(f"Données chargées: {len(df)} lignes")

# === DEBUG IMPORTANT ===
print("\nColonnes:", df.columns.tolist())
print("\nMarket names exemple:")
print(df['Market_and_Exchange_Names'].drop_duplicates().head(15).tolist())

# Traitement
def process_currency(df, keyword):
    mask = df['Market_and_Exchange_Names'].str.contains(keyword, case=False, na=False)
    sub = df[mask].copy()
    if sub.empty:
        print(f"→ Aucune donnée pour {keyword}")
        return None

    sub['Date'] = pd.to_datetime(sub['Report_Date_as_MM_DD_YYYY'], format='%m/%d/%Y', errors='coerce')
    sub = sub.sort_values('Date').drop_duplicates('Date')
    sub = sub[sub['Date'] >= pd.to_datetime(START_DATE)]

    sub['Net_Pct_OI'] = (
        (sub['NonComm_Positions_Long_All'] - sub['NonComm_Positions_Short_All'])
        / sub['Open_Interest_All'] * 100
    )

    # Style ForexFondamentale : lignes droites
    recent = sub['Net_Pct_OI'].tail(ROLLING_WINDOW)
    sub['Pct_20'] = recent.quantile(PCT_LOW)
    sub['Pct_80'] = recent.quantile(PCT_HIGH)

    return sub[['Date', 'Net_Pct_OI', 'Pct_20', 'Pct_80']].dropna(subset=['Net_Pct_OI'])

# Exécution
results = {}
for ticker, kw in CURRENCIES.items():
    data = process_currency(df, kw)
    if data is not None and len(data) > 5:
        results[ticker] = data
        last = data.iloc[-1]
        print(f"{ticker:4} | Net {last['Net_Pct_OI']:+6.1f}% | P20 {last['Pct_20']:+6.1f}% | P80 {last['Pct_80']:+6.1f}%")

print(f"\n{len(results)} devises trouvées.")

# Si toujours 0 devises, envoie-moi la sortie du debug (surtout les Market names).
if len(results) == 0:
    print("Aucune devise trouvée → envoie la sortie complète")
    exit()
