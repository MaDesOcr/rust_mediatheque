//! Un livre et son genre.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Genre {
    Roman,
    Essai,
    BandeDessinee,
}

impl Genre {
    /// Convertit un texte en genre, sans tenir compte des majuscules ni des espaces
    /// autour : « roman », « essai », « bd ». Tout autre texte donne None.
    pub fn depuis_texte(texte: &str) -> Option<Genre> {
        let texte = texte.trim().to_lowercase(); // [18] espaces et majuscules ignorés
        match texte.as_str() {
            "roman" => Some(Genre::Roman), // [3] la fonction renvoie une Option
            "essai" => Some(Genre::Essai),
            "bd" => Some(Genre::BandeDessinee),
            _ => None,
        }
    }

    /// Libellé affiché : « Roman », « Essai », « Bande dessinée ».
    pub fn libelle(&self) -> &'static str {
        match self {
            Genre::Roman => "Roman",
            Genre::Essai => "Essai",
            Genre::BandeDessinee => "Bande dessinée", // [11] toutes les variantes
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Livre {
    pub id: u32,
    pub titre: String,
    pub auteur: String,
    pub annee: u16,
    pub genre: Genre,
    pub disponible: bool,
}

impl Livre {
    /// Crée un livre, disponible par défaut.
    pub fn new(id: u32, titre: &str, auteur: &str, annee: u16, genre: Genre) -> Livre {
        Livre {
            id,
            titre: titre.to_string(),
            auteur: auteur.to_string(),
            annee,
            genre,
            disponible: true,
        }
    }

    /// Vrai si le livre est disponible.
    pub fn est_disponible(&self) -> bool {
        // [2] une méthode reçoit &self
        self.disponible
    }

    /// Âge du livre en années, par rapport à `annee_courante`
    /// (0 si l'année de parution est postérieure).
    pub fn age(&self, annee_courante: u16) -> u16 {
        annee_courante.saturating_sub(self.annee) // [19] pas de soustraction négative sur un u16
    }
}

/// Affichage : « n°3 « Dune » de Frank Herbert (1965, Roman) ».
impl fmt::Display for Livre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "n°{} « {} » de {} ({}, {})",
            self.id,
            self.titre, // [20] titre puis auteur
            self.auteur,
            self.annee,
            self.genre.libelle()
        ) // [4] pas de « ; » : write! renvoie le fmt::Result
    }
}