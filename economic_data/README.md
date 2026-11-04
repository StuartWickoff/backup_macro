# Economic Data - Macro Journal

> **Dossier** : Données économiques historiques pour les 8 devises (EUR, GBP, JPY, CHF, CAD, AUD, NZD, CNY).
> **Période** : 36 derniers mois + 12 derniers trimestres.
> **Source** : Priorité à Investing.com, puis sources officielles.

---

## 📁 **Structure des Fichiers**

```
economic_data/
├── SOURCES.md          # Documentation complète des sources (URLs, définitions, etc.)
├── README.md           # Ce fichier
├── CONTROL_SUMMARY.md # Résumé de contrôle (à générer après import)
├── EUR/
│   ├── history.csv     # Données historiques (format : series_id,date,value,value_secondary)
│   └── SOURCES.md      # Sources spécifiques à l'EUR (optionnel)
├── GBP/
│   ├── history.csv
│   └── SOURCES.md
├── JPY/
│   ├── history.csv
│   └── SOURCES.md
├── CHF/
│   ├── history.csv
│   └── SOURCES.md
├── CAD/
│   ├── history.csv
│   └── SOURCES.md
├── AUD/
│   ├── history.csv
│   └── SOURCES.md
├── NZD/
│   ├── history.csv
│   └── SOURCES.md
└── CNY/
    ├── history.csv
    └── SOURCES.md
```

---

## 📄 **Format des Fichiers CSV**

### **En-tête obligatoire**
```csv
series_id,date,value,value_secondary
```

### **Exemple (EUR)**
```csv
series_id,date,value,value_secondary
policy_rate,2024-01-01,4.0,
hicp_headline_yoy,2024-01-01,2.8,
hicp_core_yoy,2024-01-01,3.1,
ppi_yoy,2024-01-01,1.2,
unemployment_rate,2024-01-01,6.4,
real_gdp_growth,2024-01-01,0.1,
sp_global_manufacturing_pmi,2024-01-01,46.8,
sp_global_services_pmi,2024-01-01,48.5,
policy_rate,2024-02-01,4.0,
hicp_headline_yoy,2024-02-01,2.6,
... (36 mois)
```

### **Règles de Formatage**
1. **Dates mensuelles** : Toujours au format `YYYY-MM-01` (ex: `2024-01-01` pour janvier 2024).
2. **Dates trimestrielles** : Toujours au format `YYYY-01-01`, `YYYY-04-01`, `YYYY-07-01`, ou `YYYY-10-01`.
3. **Valeurs numériques** : Utiliser le **point** comme séparateur décimal (ex: `2.8`, pas `2,8`).
4. **Valeurs manquantes** : Laisser `value` vide (ne pas inventer de données).
5. **Séries séparées** : Chaque indicateur a ses propres lignes (pas de fusion).

---

## 🔄 **Workflows**

### **1. Téléchargement des Données**
Consulte `SOURCES.md` pour les URLs des sources officielles.

**Étapes** :
1. Pour chaque devise, télécharger les données depuis les sources indiquées.
2. Formater les données selon le format CSV ci-dessus.
3. Sauvegarder dans `economic_data/<currency>/history.csv`.

### **2. Import en Base de Données**
Utiliser le script Rust fourni pour importer les CSV dans SQLite :

```bash
# Depuis la racine du projet
cargo run --bin import-economic-data -- /chemin/vers/economic_data
```

### **3. Génération du Résumé de Contrôle**
```bash
cargo run --bin generate-control-summary -- /chemin/vers/economic_data
```

---

## 🛠️ **Outils Disponibles**

### **1. Script Rust : `import-economic-data`**
- **Fonction** : Importe les fichiers CSV dans la base SQLite de Macro Journal.
- **Lieu** : `tools/import-economic-data/` (à créer).

### **2. Script Rust : `generate-control-summary`**
- **Fonction** : Génère un rapport de contrôle (`CONTROL_SUMMARY.md`) avec :
  - Nombre d'observations par série.
  - Première et dernière date.
  - Périodes manquantes.
  - Alertes sur les révisions.

### **3. Script Python : `download-economic-data.py`**
- **Fonction** : Télécharge automatiquement les données depuis les APIs publiques.
- **Lieu** : `tools/download-economic-data/` (à créer).
- **Usage** :
  ```bash
  python3 tools/download-economic-data/download.py --api-key FRED_KEY
  ```

---

## 📊 **Identifiants des Séries**

| Devise | Indicateur | series_id |
|--------|------------|-----------|
| **Toutes** | Taux directeur | `policy_rate` |
| **Toutes** | PPI (YoY) | `ppi_yoy` |
| **Toutes** | Chômage | `unemployment_rate` |
| **Toutes** | PIB réel | `real_gdp_growth` |
| **Toutes** | PMI Manufacturier | `sp_global_manufacturing_pmi` (sauf CNY) |
| **Toutes** | PMI Services | `sp_global_services_pmi` (sauf CNY) |
| **EUR** | Inflation (globale) | `hicp_headline_yoy` |
| **EUR** | Inflation (core) | `hicp_core_yoy` |
| **GBP, JPY, CHF, NZD, CNY** | Inflation (globale) | `cpi_headline_yoy` |
| **GBP, JPY, CHF, NZD, CNY** | Inflation (core) | `cpi_core_yoy` |
| **CAD** | CPI Common | `cpi_common` |
| **CAD** | CPI Trimmed | `cpi_trimmed` |
| **CAD** | CPI Median | `cpi_median` |
| **AUD** | CPI Trimmed Mean | `cpi_trimmed_mean` |
| **AUD** | CPI Weighted Median | `cpi_weighted_median` |
| **CNY** | PMI Manufacturier | `caixin_manufacturing_pmi` |
| **CNY** | PMI Services | `caixin_services_pmi` |

---

## 🔍 **Vérification des Données**

Avant d'importer :
- [ ] Vérifier que les dates sont au bon format (`YYYY-MM-01`).
- [ ] Vérifier que les valeurs sont numériques (avec point décimal).
- [ ] Vérifier qu'il n'y a pas de valeurs inventées.
- [ ] Vérifier que les séries couvrent bien 36 mois / 12 trimestres.

---

## 📝 **Journal des Modifications**

| Date | Action | Responsable |
|------|--------|-------------|
| 2026-11-03 | Création de la structure des dossiers | Assistant |
| 2026-11-03 | Documentation des sources (SOURCES.md) | Assistant |
| ? | Téléchargement des données | À faire |
| ? | Import en base de données | À faire |
| ? | Génération du résumé de contrôle | À faire |

---

## 🎯 **Prochaines Étapes**

1. **Télécharger les données** depuis les sources officielles (voir `SOURCES.md`).
2. **Formater les données** selon le format CSV spécifié.
3. **Tester l'import** avec le script Rust.
4. **Générer le résumé de contrôle**.
5. **Intégrer les données** dans ton application Macro Journal.

---

## 🆘 **Aide**

Si tu as des questions sur :
- Le format des données.
- Les sources à utiliser.
- L'import en base de données.
- L'intégration dans Macro Journal.

**Consulte `SOURCES.md` pour les détails techniques.**
