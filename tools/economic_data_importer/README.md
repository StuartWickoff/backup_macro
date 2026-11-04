# Economic Data Importer

> **Outil Rust** pour importer les données économiques dans Macro Journal

---

## 📋 **Description**

Cet outil permet d'importer des fichiers CSV contenant des données économiques historiques dans la base de données SQLite de **Macro Journal**.

**Fonctionnalités** :
- ✅ Import des données depuis des fichiers CSV structurés
- ✅ Validation des données (format, séries, dates)
- ✅ Génération automatique d'un rapport de contrôle
- ✅ Intégration avec la base SQLite existante

---

## 📁 **Structure des Données**

```
economic_data/
├── EUR/
│   └── history.csv      # Données au format: series_id,date,value,value_secondary
├── GBP/
│   └── history.csv
├── ... (JPY, CHF, CAD, AUD, NZD, CNY)
└── SOURCES.md           # Documentation des sources
```

### **Format CSV**

```csv
series_id,date,value,value_secondary
policy_rate,2024-01-01,4.0,
hicp_headline_yoy,2024-01-01,2.8,
hicp_core_yoy,2024-01-01,3.1,
...
```

**Règles** :
- **Dates mensuelles** : `YYYY-MM-01` (ex: `2024-01-01`)
- **Dates trimestrielles** : `YYYY-01-01`, `YYYY-04-01`, `YYYY-07-01`, `YYYY-10-01`
- **Valeurs numériques** : Point comme séparateur décimal (ex: `2.8`)
- **Valeurs manquantes** : Laisser vide (ne pas inventer)

---

## 🚀 **Installation**

### **1. Ajouter au workspace**

Le projet est déjà ajouté au workspace Cargo (`Cargo.toml`).

### **2. Compiler**

```bash
cd /path/to/macro_journal
cargo build --package economic-data-importer --release
```

Le binaire sera généré dans :
```
target/release/economic-data-importer
```

---

## 🎯 **Utilisation**

### **1. Importer les données**

```bash
# Depuis la racine du projet
cargo run --package economic-data-importer -- import \
    --data-dir ./economic_data \
    --db-path ~/.local/share/macro-journal/macro_journal.db
```

**Options** :
- `--data-dir` : Chemin vers le dossier `economic_data` (obligatoire)
- `--db-path` : Chemin vers la base SQLite (optionnel, défaut: XDG)
- `--overwrite` : Écraser les données existantes (optionnel, défaut: false)

**Exemple** :
```bash
cargo run --package economic-data-importer -- import \
    --data-dir ./economic_data \
    --db-path ~/.local/share/macro-journal/macro_journal.db \
    --overwrite
```

---

### **2. Valider les données**

Vérifie que les fichiers CSV sont correctement formatés :

```bash
cargo run --package economic-data-importer -- validate \
    --data-dir ./economic_data
```

**Ce que ça vérifie** :
- ✅ Format des dates (`YYYY-MM-DD`)
- ✅ Séries attendues par devise
- ✅ Couverture temporelle (36 mois / 12 trimestres)
- ✅ Valeurs numériques valides

---

### **3. Générer le résumé de contrôle**

Crée un rapport `CONTROL_SUMMARY.md` avec :
- Nombre d'observations par série
- Première et dernière date
- Périodes manquantes
- Pourcentage de couverture

```bash
cargo run --package economic-data-importer -- generate-summary \
    --data-dir ./economic_data \
    --output ./economic_data/CONTROL_SUMMARY.md
```

---

## 📊 **Séries Économiques**

### **Toutes les devises**
| Indicateur | series_id | Type | Source |
|------------|-----------|------|--------|
| Taux directeur | `policy_rate` | Mensuel | Banque centrale |
| PPI (YoY) | `ppi_yoy` | Mensuel | Statistiques nationales |
| Chômage | `unemployment_rate` | Mensuel | Statistiques nationales |
| PIB réel | `real_gdp_growth` | Trimestriel | Statistiques nationales |
| PMI Manufacturier | `sp_global_manufacturing_pmi` | Mensuel | S&P Global |
| PMI Services | `sp_global_services_pmi` | Mensuel | S&P Global |

### **EUR (Zone Euro)**
| Indicateur | series_id | Source |
|------------|-----------|--------|
| Inflation (globale) | `hicp_headline_yoy` | Eurostat |
| Inflation (core) | `hicp_core_yoy` | Eurostat |

### **GBP, JPY, CHF, NZD, CNY**
| Indicateur | series_id | Source |
|------------|-----------|--------|
| Inflation (globale) | `cpi_headline_yoy` | Statistiques nationales |
| Inflation (core) | `cpi_core_yoy` | Statistiques nationales |

### **CAD (Canada)**
| Indicateur | series_id | Source |
|------------|-----------|--------|
| CPI Common | `cpi_common` | StatCan |
| CPI Trimmed | `cpi_trimmed` | StatCan |
| CPI Median | `cpi_median` | StatCan |

### **AUD (Australie)**
| Indicateur | series_id | Source |
|------------|-----------|--------|
| CPI Trimmed Mean | `cpi_trimmed_mean` | ABS |
| CPI Weighted Median | `cpi_weighted_median` | ABS |

### **CNY (Chine)**
| Indicateur | series_id | Source |
|------------|-----------|--------|
| PMI Manufacturier | `caixin_manufacturing_pmi` | Caixin |
| PMI Services | `caixin_services_pmi` | Caixin |

---

## 🔍 **Validation**

L'outil vérifie automatiquement :

### **✅ Format des dates**
- Mensuel : `YYYY-MM-01` (ex: `2024-01-01`)
- Trimestriel : `YYYY-01-01`, `YYYY-04-01`, `YYYY-07-01`, `YYYY-10-01`

### **✅ Séries attendues**
Chaque devise doit avoir :
- 8 séries (7 mensuelles + 1 trimestrielle)
- 36 observations mensuelles
- 12 observations trimestrielles

### **✅ Valeurs numériques**
- Séparateur décimal : **point** (`.`) pas virgule
- Pas de valeurs inventées

---

## 🛠️ **Intégration avec Macro Journal**

Une fois les données importées dans SQLite, tu peux :

### **1. Créer une API pour accéder aux données**

Ajoute un endpoint dans `apps/server/src/main.rs` :

```rust
async fn get_economic_data(
    State(state): State<AppState>,
    Path(currency): Path<String>,
) -> Result<Json<Vec<EconomicDataResponse>>, ApiFailure> {
    let records = sqlx::query_as::<_, EconomicDataResponse>(
        "SELECT series_id, date, value, value_secondary FROM economic_data 
         WHERE currency_code = ? ORDER BY series_id, date",
    )
    .bind(currency)
    .fetch_all(&state.pool)
    .await
    .map_err(ApiFailure::database)?;

    Ok(Json(records))
}
```

### **2. Afficher les données dans le frontend**

Utilise Leptos pour créer une page de visualisation :
- Graphiques (via un crate comme `leptos-chartjs`)
- Tableaux de données
- Filtres par devise et indicateur

---

## 📝 **Exemple Complet**

### **Étape 1 : Préparer les données**

Télécharger les données depuis les sources officielles (voir `economic_data/SOURCES.md`) :

```bash
# Exemple pour EUR
mkdir -p economic_data/EUR
# Télécharger les données et les sauvegarder dans economic_data/EUR/history.csv
```

### **Étape 2 : Valider**

```bash
cargo run --package economic-data-importer -- validate --data-dir ./economic_data
```

### **Étape 3 : Importer**

```bash
cargo run --package economic-data-importer -- import \
    --data-dir ./economic_data \
    --db-path ~/.local/share/macro-journal/macro_journal.db
```

### **Étape 4 : Générer le résumé**

```bash
cargo run --package economic-data-importer -- generate-summary \
    --data-dir ./economic_data \
    --output ./economic_data/CONTROL_SUMMARY.md
```

---

## ❓ **FAQ**

### **Q: Où trouver les données sources ?**
R: Voir le fichier `economic_data/SOURCES.md` pour les URLs des sources officielles.

### **Q: Comment formater les dates ?**
R: Utiliser `YYYY-MM-01` pour les données mensuelles (ex: `2024-01-01` pour janvier 2024).

### **Q: Que faire si une série n'a pas 36 observations ?**
R: L'outil signalera l'erreur. Complète les données manquantes depuis les sources officielles.

### **Q: Puis-je utiliser Investing.com ?**
R: **Non recommandé** pour l'accès automatisé (protection anti-robot). Préfère les sources officielles.

---

## 🔗 **Liens Utiles**

- [SOURCES.md](economic_data/SOURCES.md) - Documentation complète des sources
- [README.md](economic_data/README.md) - Structure des données
- [Macro Journal README](../README.md) - Documentation principale
