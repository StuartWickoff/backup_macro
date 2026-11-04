#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Audit automatisé de data_eco/ — Journal macroéconomique.

Contrôles :
  1.  structure   : en-tête exact, 4 colonnes, valeurs numériques
  2.  schéma      : value_secondary VIDE partout (une mesure = un series_id)
  3.  dates       : ISO, jour = 01 ; séries trimestrielles au 1er jour du trimestre
  4.  unicité     : aucun doublon (series_id, date)
  5.  fenêtre     : 2023-11 → 2026-10 (mensuel) / 2023T4 → 2026T3 (trimestriel)
  6.  inventaire  : séries attendues présentes, aucune série inattendue
  7.  couverture  : dates manquantes exactes par série
  8.  plausibilité: bornes de valeurs par type de série
  9.  points de contrôle externes (vérifiés contre publications officielles)
  10. cohérence _fetch_log.json ↔ CSV

Usage : python3 audit.py   (depuis le répertoire data_eco/)
Écrit le rapport AUDIT.md et renvoie 0 si aucun échec, 1 sinon.
"""
import csv
import json
import re
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
CURS = ['eur', 'gbp', 'jpy', 'chf', 'cad', 'aud', 'nzd', 'cny']
MENS = [f'{y:04d}-{m:02d}' for y in range(2023, 2027) for m in range(1, 13)
        if (y > 2023 or m >= 11) and (y < 2026 or m <= 10)]
TRIM = [f'{y:04d}-{3 * q - 2:02d}' for y in (2023, 2024, 2025, 2026) for q in (1, 2, 3, 4)
        if not (y == 2023 and q < 4) and not (y == 2026 and q > 3)]

EXPECTED = {
    'eur': {'policy_rate', 'hicp_headline_yoy', 'hicp_core_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'sp_global_manufacturing_pmi', 'sp_global_services_pmi'},
    'gbp': {'policy_rate', 'cpi_headline_yoy', 'cpi_core_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'sp_global_manufacturing_pmi', 'sp_global_services_pmi'},
    'jpy': {'policy_rate', 'cpi_headline_yoy', 'cpi_core_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'sp_global_manufacturing_pmi', 'sp_global_services_pmi'},
    'chf': {'policy_rate', 'cpi_headline_yoy', 'cpi_core_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy'},
    'cad': {'policy_rate', 'cpi_trimmed_yoy', 'cpi_median_yoy', 'cpi_common',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'sp_global_manufacturing_pmi', 'sp_global_services_pmi'},
    'aud': {'policy_rate', 'cpi_trimmed_yoy', 'cpi_weighted_median_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'sp_global_manufacturing_pmi', 'sp_global_services_pmi'},
    'nzd': {'policy_rate', 'cpi_headline_yoy', 'cpi_core_proxy_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy'},
    'cny': {'policy_rate', 'cpi_headline_yoy', 'cpi_core_yoy', 'ppi_yoy',
            'unemployment_rate', 'gdp_qoq', 'gdp_yoy',
            'ratingdog_manufacturing_pmi', 'ratingdog_services_pmi'},
}
QUARTERLY = {
    'chf': {'unemployment_rate', 'gdp_qoq', 'gdp_yoy'},
    'aud': {'cpi_trimmed_yoy', 'cpi_weighted_median_yoy', 'ppi_yoy', 'gdp_qoq', 'gdp_yoy'},
    'nzd': {'cpi_headline_yoy', 'cpi_core_proxy_yoy', 'ppi_yoy', 'unemployment_rate',
            'gdp_qoq', 'gdp_yoy'},
    **{c: {'gdp_qoq', 'gdp_yoy'} for c in ('eur', 'gbp', 'jpy', 'cad', 'cny')},
}
RANGES = {  # (min, max) indicatifs de plausibilité
    'policy_rate': (-1.0, 25.0), 'unemployment_rate': (0.0, 30.0),
    'gdp_qoq': (-15.0, 15.0), 'gdp_yoy': (-25.0, 25.0),
    'ppi_yoy': (-30.0, 40.0),
}
CONTROL_POINTS = [  # (devise, série, date, valeur attendue, référence)
    ('eur', 'hicp_headline_yoy', '2026-10-01', 4.0, 'Eurostat flash 04/11/2026'),
    ('eur', 'hicp_core_yoy', '2026-10-01', 2.5, 'Eurostat flash 04/11/2026'),
    ('gbp', 'cpi_headline_yoy', '2026-09-01', 3.5, 'ONS 21/10/2026'),
    ('gbp', 'cpi_core_yoy', '2026-09-01', 2.8, 'ONS 21/10/2026'),
    ('cny', 'cpi_headline_yoy', '2026-09-01', 0.9, 'NBS'),
    ('cny', 'cpi_core_yoy', '2026-09-01', 1.0, 'NBS'),
    ('cny', 'ppi_yoy', '2026-09-01', 5.0, 'NBS'),
    ('cny', 'unemployment_rate', '2026-09-01', 5.2, 'NBS'),
    ('cny', 'gdp_qoq', '2026-07-01', 1.2, 'NBS T3 2026'),
    ('cny', 'gdp_yoy', '2026-07-01', 4.5, 'NBS T3 2026'),
    ('chf', 'cpi_headline_yoy', '2026-10-01', 1.5, 'OFS 03/11/2026'),
    ('aud', 'policy_rate', '2026-10-01', 4.6, 'RBA (4,85 % le 04/11, hors fenêtre)'),
    ('cad', 'unemployment_rate', '2026-09-01', 6.5, 'StatCan LFS 09/10/2026'),
    ('nzd', 'unemployment_rate', '2026-07-01', 5.7, 'Stats NZ LMS 04/11/2026'),
    ('jpy', 'sp_global_services_pmi', '2026-08-01', 52.5, 'S&P Global (corrigé)'),
    ('aud', 'sp_global_manufacturing_pmi', '2026-10-01', 49.6, 'S&P Global 01/11/2026'),
    ('aud', 'sp_global_services_pmi', '2026-10-01', 48.5, 'S&P Global 03/11/2026'),
    ('cny', 'policy_rate', '2026-10-01', 1.4, 'PBOC — inchangé depuis 07/05/2025'),
]

failures, warnings, report = [], [], []


def fail(msg):
    failures.append(msg)


def warn(msg):
    warnings.append(msg)


def check(cond, msg):
    if not cond:
        fail(msg)
    return cond


# ------------------------------------------------------------------ chargement
DATA = {}
for cur in CURS:
    p = BASE / cur / 'history.csv'
    if not check(p.exists(), f'{cur}: fichier manquant {p}'):
        DATA[cur] = []
        continue
    with p.open(newline='') as f:
        raw = list(csv.reader(f))
    check(raw and raw[0] == ['series_id', 'date', 'value', 'value_secondary'],
          f'{cur}: en-tête invalide : {raw[0] if raw else "vide"}')
    DATA[cur] = [r for r in raw[1:] if r]

total = sum(len(v) for v in DATA.values())
report.append(f'Total : **{total} observations** sur 8 devises.')

# ------------------------------------------------------------------ contrôles 1-8
for cur in CURS:
    rows = DATA[cur]
    seen = set()
    months_by_sid = {}
    for r in rows:
        sid, date, val, sec = (r + [''] * 4)[:4]
        loc = f'{cur}/{sid}/{date}'
        check(len(r) == 4, f'{loc}: {len(r)} colonnes (4 attendues)')
        check(re.match(r'^\d{4}-\d{2}-\d{2}$', date), f'{loc}: date non ISO ({date})')
        check(date.endswith('-01'), f'{loc}: jour != 01 ({date})')
        check(sec.strip() == '', f'{loc}: value_secondary non vide ({sec!r})')
        try:
            float(val)
        except ValueError:
            fail(f'{loc}: valeur non numérique ({val!r})')
            continue
        k = (sid, date)
        check(k not in seen, f'{loc}: doublon series_id+date')
        seen.add(k)
        months_by_sid.setdefault(sid, []).append(date[:7])
        if sid in QUARTERLY.get(cur, set()):
            check(date[5:7] in ('01', '04', '07', '10'),
                  f'{loc}: série trimestrière datée hors 1er jour de trimestre ({date})')
            check(date[:7] in TRIM, f'{loc}: date hors fenêtre trimestrielle ({date})')
            rg = RANGES.get('gdp_qoq' if sid.startswith('gdp') else sid)
        else:
            check(date[:7] in MENS, f'{loc}: date hors fenêtre mensuelle ({date})')
            rg = RANGES.get(sid)
        if sid.endswith('_pmi'):
            check(25.0 <= float(val) <= 75.0, f'{loc}: PMI improbable ({val})')
        elif rg:
            check(rg[0] <= float(val) <= rg[1], f'{loc}: valeur hors bornes {rg} ({val})')
    # inventaire
    got = set(months_by_sid)
    for sid in sorted(EXPECTED[cur] - got):
        fail(f'{cur}: série attendue absente : {sid}')
    for sid in sorted(got - EXPECTED[cur]):
        fail(f'{cur}: série inattendue : {sid}')

# ------------------------------------------------------------------ points de contrôle
report.append('\n## Points de contrôle externes\n')
report.append('| Devise | Série | Date | Valeur | Attendu | Référence | Statut |')
report.append('|---|---|---|---|---|---|---|')
for cur, sid, date, expected, ref in CONTROL_POINTS:
    rows = [r for r in DATA.get(cur, []) if r[0] == sid and r[1] == date]
    if not rows:
        fail(f'point de contrôle manquant : {cur}/{sid}/{date}')
        report.append(f'| {cur.upper()} | `{sid}` | {date} | — | {expected} | {ref} | ÉCHEC |')
        continue
    v = float(rows[0][2])
    ok = abs(v - expected) < 0.051
    if not ok:
        fail(f'point de contrôle : {cur}/{sid}/{date} = {v}, attendu {expected} ({ref})')
    report.append(f'| {cur.upper()} | `{sid}` | {date} | {v:g} | {expected:g} | {ref} | '
                  f'{"OK" if ok else "ÉCHEC"} |')

# ------------------------------------------------------------------ couverture
report.append('\n## Couverture (dates manquantes exactes)\n')
for cur in CURS:
    report.append(f'### {cur.upper()}')
    report.append('| Série | Obs. | Dernière | Manquant |')
    report.append('|---|---|---|---|')
    sids = sorted({r[0] for r in DATA[cur]})
    for sid in sids:
        months = [r[1][:7] for r in DATA[cur] if r[0] == sid]
        if sid in QUARTERLY.get(cur, set()):
            qs = {f'{m[:4]}-{3 * ((int(m[5:7]) - 1) // 3) + 1:02d}' for m in months}
            missing = [f'{t[:4]}-T{(int(t[5:7]) - 1) // 3 + 1}' for t in TRIM if t not in qs]
        else:
            missing = [m for m in MENS if m not in set(months)]
        last = max(months) if months else '—'
        report.append(f'| `{sid}` | {len(months)} | {last} | {", ".join(missing) if missing else "—"} |')
    report.append('')

# ------------------------------------------------------------------ journal
report.append('\n## Cohérence `_fetch_log.json` ↔ CSV\n')
logp = BASE / '_fetch_log.json'
if not logp.exists():
    fail('_fetch_log.json absent')
else:
    log = json.loads(logp.read_text())
    entries = {}
    for e in log:
        k = (e.get('currency', '').lower(), e.get('series_id'))
        if k in entries:
            fail(f'journal : entrée dupliquée {k}')
        entries[k] = e
    for cur in CURS:
        for sid in sorted({r[0] for r in DATA[cur]}):
            e = entries.get((cur, sid))
            if not check(e is not None, f'journal : entrée manquante pour {cur}/{sid}'):
                continue
            n = len([r for r in DATA[cur] if r[0] == sid])
            dates = sorted(r[1] for r in DATA[cur] if r[0] == sid)
            check(e.get('n_obs') == n, f'journal {cur}/{sid}: n_obs={e.get("n_obs")} vs CSV {n}')
            check(e.get('first') == dates[0], f'journal {cur}/{sid}: first={e.get("first")} vs {dates[0]}')
            check(e.get('last') == dates[-1], f'journal {cur}/{sid}: last={e.get("last")} vs {dates[-1]}')
    for k in entries:
        if k[0] in CURS and k[1] not in {r[0] for r in DATA.get(k[0], [])}:
            fail(f'journal : entrée orpheline {k}')
    report.append(f'{len(entries)} entrées, une par série ; n_obs / first / last cohérents avec les CSV.'
                  if not any('journal' in f for f in failures)
                  else 'Incohérences détectées (voir échecs).')

# ------------------------------------------------------------------ rapport
head = ['# AUDIT.md — Audit automatisé de `data_eco/`', '',
        f'Exécuté le 4 novembre 2026 (2ᵉ passe). Script : `audit.py` (aucune dépendance externe).',
        '', '## Résultat global', '',
        f'- **Échecs : {len(failures)}**',
        f'- Avertissements : {len(warnings)}',
        '- Contrôles : structure, schéma (value_secondary vide), dates (1ᵉʳ jour du'
        ' mois/trimestre), unicité, fenêtre, inventaire des séries, plausibilité,'
        ' points de contrôle externes, cohérence du journal de collecte.',
        '']
if failures:
    head.append('## Échecs\n')
    head.extend(f'- ❌ {f}' for f in failures)
    head.append('')
if warnings:
    head.append('## Avertissements\n')
    head.extend(f'- ⚠️ {w}' for w in warnings)
    head.append('')
(BASE / 'AUDIT.md').write_text('\n'.join(head + report) + '\n')

print('\n'.join(head[:12]))
print(f'Rapport complet : {BASE / "AUDIT.md"}')
sys.exit(1 if failures else 0)
