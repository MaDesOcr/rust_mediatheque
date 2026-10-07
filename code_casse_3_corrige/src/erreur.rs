//! Les erreurs que peut renvoyer la médiathèque.

use std::fmt; // [1] import nécessaire pour fmt::Display, fmt::Formatter, fmt::Result

#[derive(Debug, PartialEq)]
pub enum ErreurMediatheque {
    /// Aucun livre ne porte cet identifiant.
    LivreIntrouvable(u32),
    /// Le livre (dont on donne le titre) est déjà emprunté.
    DejaEmprunte(String),
    /// Le livre (dont on donne le titre) n'est pas emprunté : impossible de le rendre.
    PasEmprunte(String),
    /// Le nombre maximal d'emprunts simultanés est atteint.
    LimiteAtteinte,
    /// Une ligne du fichier est mal formée (on donne la ligne).
    FormatInvalide(String),
    /// Le fichier n'a pas pu être lu (on donne le chemin).
    Fichier(String),
}

impl fmt::Display for ErreurMediatheque {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErreurMediatheque::LivreIntrouvable(id) => write!(f, "aucun livre n°{id}"),
            ErreurMediatheque::DejaEmprunte(titre) => write!(f, "« {titre} » est déjà emprunté"),
            ErreurMediatheque::PasEmprunte(titre) => write!(f, "« {titre} » n'est pas emprunté"),
            ErreurMediatheque::LimiteAtteinte => write!(f, "limite d'emprunts atteinte"),
            ErreurMediatheque::FormatInvalide(ligne) => write!(f, "ligne invalide : {ligne}"),
            ErreurMediatheque::Fichier(chemin) => write!(f, "impossible de lire {chemin}"),
        }
    }
}
