# Macro Journal — patch barre latérale

Cette mise à jour ajoute un bouton dans la barre supérieure pour masquer ou afficher la navigation latérale.

## Installation

Depuis le répertoire racine de ton projet `macro-journal`, extrais cette archive en conservant les chemins :

```bash
unzip -o macro-journal-sidebar-toggle-patch.zip -d .
```

Les deux fichiers concernés sont :

- `apps/web/src/main.rs`
- `apps/web/assets/styles.css`

La mise à jour conserve le correctif du gestionnaire `Actualiser` (`on:click=move |_| load_entries()`). Elle ne touche pas à la base SQLite.

## Utilisation

Le bouton avec le chevron / menu se trouve en haut à gauche, dans la barre supérieure. Un clic masque la navigation ; un second clic la réaffiche. Le panneau des notes à l'intérieur du journal n'est pas affecté.

L'état du bouton est en mémoire et revient à l'état affiché par défaut après rechargement de la page.

## Relancer

Arrête l'ancien `run-dev.sh` avec `Ctrl+C`, puis depuis la racine du projet :

```bash
./run-dev.sh
```

Ouvre `http://127.0.0.1:8080`.
