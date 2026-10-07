//! Médiathèque : gestion d'un catalogue de livres et des emprunts — CORRIGÉ.
//! Chaque correction est repérée par son numéro [n] (voir corrige.md).

pub mod erreur;
pub mod livre;
pub mod mediatheque; // [14] module public, sinon inaccessible depuis main.rs et les tests
