//! Exercice « Code cassé 3 » — Médiathèque : toutes les notions des jours 1 à 3.
//!
//! Le projet contient 27 erreurs, réparties dans src/ (lib.rs, erreur.rs, livre.rs,
//! mediatheque.rs et main.rs) :
//!   - 17 empêchent la compilation ;
//!   - 10 sont des erreurs de logique : le code compile, mais les tests échouent ou plantent.
//!
//! Méthode :
//!   1. `cargo build` : corrigez les erreurs de compilation, une à la fois.
//!      Elles apparaissent par vagues : la bibliothèque d'abord, puis main.rs.
//!      Lisez aussi les avertissements (warnings) : certains sont de précieux indices ;
//!   2. `cargo test` : corrigez la logique jusqu'à ce que les 19 tests passent
//!      (`cargo test t07` pour relancer un seul test) ;
//!   3. `cargo run` : le programme charge donnees/catalogue.csv et simule des emprunts.
//!
//! Règles : ne modifiez ni les tests (tests/verification.rs), ni le fichier de données,
//! ni les signatures des fonctions publiques (noms, paramètres, types de retour).
//! La documentation `///` de chaque fonction décrit le comportement attendu.

pub mod erreur;
pub mod livre;
mod mediatheque;
