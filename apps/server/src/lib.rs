//! Bibliothèque du serveur Macro Journal.
//!
//! Regroupe les briques non-HTTP réutilisées par le binaire `main.rs` :
//!
//! - [`config`] : résolution des chemins de stockage (env > XDG > défauts),
//! - [`storage`] : ouverture/initialisation sécurisée de la base SQLite
//!   (sentinelle `.initialized` anti-recréation silencieuse),
//! - [`backup`] : snapshots SQLite vérifiés (`VACUUM INTO` + contrôle
//!   d'intégrité).

pub mod backup;
pub mod config;
pub mod storage;
