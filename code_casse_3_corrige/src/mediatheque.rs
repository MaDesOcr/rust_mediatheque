//! Le catalogue et les opérations d'emprunt.

use crate::erreur::ErreurMediatheque;
use crate::livre::{Genre, Livre};
use std::fs;

/// Nombre maximal de livres empruntés en même temps.
pub const LIMITE_EMPRUNTS: u32 = 3;

#[derive(Debug, Default)]
pub struct Mediatheque {
    pub livres: Vec<Livre>,
    pub nb_emprunts: u32,
}

impl Mediatheque {
    /// Médiathèque vide.
    pub fn new() -> Mediatheque {
        Mediatheque {
            livres: Vec::new(),
            nb_emprunts: 0,
        }
    }

    /// Ajoute un livre au catalogue et renvoie son identifiant.
    pub fn ajouter(&mut self, livre: Livre) -> u32 {
        let id = livre.id; // [12] lire l'id avant que push ne déplace le livre
        self.livres.push(livre);
        id
    }

    /// Le livre portant cet identifiant, s'il existe.
    pub fn trouver(&self, id: u32) -> Option<&Livre> {
        for livre in &self.livres {
            if livre.id == id {
                return Some(livre);
            }
        }
        None
    }

    /// Le livre portant cet identifiant, modifiable, s'il existe.
    pub fn trouver_mut(&mut self, id: u32) -> Option<&mut Livre> {
        for livre in &mut self.livres {
            // [5] &mut pour obtenir des &mut Livre
            if livre.id == id {
                return Some(livre);
            }
        }
        None
    }

    /// Nombre de livres disponibles.
    pub fn nb_disponibles(&self) -> u32 {
        let mut n = 0; // [6] u32, comme le type de retour
        for livre in &self.livres {
            if livre.est_disponible() {
                // [21] compter les disponibles, pas les empruntés
                n += 1;
            }
        }
        n
    }

    /// Emprunte un livre. Erreurs, dans cet ordre de vérification :
    /// LimiteAtteinte si LIMITE_EMPRUNTS livres sont déjà empruntés,
    /// LivreIntrouvable si l'id n'existe pas, DejaEmprunte si le livre n'est pas disponible.
    pub fn emprunter(&mut self, id: u32) -> Result<(), ErreurMediatheque> {
        if self.nb_emprunts >= LIMITE_EMPRUNTS {
            // [22] >= : la limite est atteinte à 3
            return Err(ErreurMediatheque::LimiteAtteinte);
        }
        // [7] trouver_mut renvoie une Option : ok_or la transforme en Result, ? propage l'erreur
        let livre = self
            .trouver_mut(id)
            .ok_or(ErreurMediatheque::LivreIntrouvable(id))?;
        if !livre.disponible {
            return Err(ErreurMediatheque::DejaEmprunte(livre.titre.clone()));
        }
        livre.disponible = false;
        self.nb_emprunts += 1; // [23] compter seulement un emprunt réussi
        Ok(()) // [8] la fonction renvoie un Result
    }

    /// Rend un livre. Erreurs : LivreIntrouvable, ou PasEmprunte si le livre est disponible.
    pub fn rendre(&mut self, id: u32) -> Result<(), ErreurMediatheque> {
        let livre = match self.trouver_mut(id) {
            Some(l) => l,
            None => return Err(ErreurMediatheque::LivreIntrouvable(id)), // [9] return : quitter la fonction
        };
        if livre.disponible {
            // [24] sans ce test, nb_emprunts passerait sous 0 (panique)
            return Err(ErreurMediatheque::PasEmprunte(livre.titre.clone()));
        }
        livre.disponible = true;
        self.nb_emprunts -= 1;
        Ok(())
    }

    /// Le plus ancien livre du catalogue (le premier rencontré en cas d'égalité).
    pub fn plus_ancien(&self) -> Option<&Livre> {
        let mut resultat: Option<&Livre> = None; // [13] mut
        for livre in &self.livres {
            match resultat {
                Some(r) if r.annee <= livre.annee => {} // [25] garder r s'il est plus ancien
                _ => resultat = Some(livre),
            }
        }
        resultat
    }
}

/// Erreur de format pour une ligne (fonction utilitaire interne au module).
fn ligne_invalide(ligne: &str) -> ErreurMediatheque {
    ErreurMediatheque::FormatInvalide(ligne.to_string())
}

/// Analyse une ligne « id;titre;auteur;annee;genre ».
/// Les espaces autour de chaque champ sont ignorés.
/// Toute ligne mal formée (nombre de champs différent de 5, nombre ou genre invalide)
/// donne FormatInvalide avec la ligne d'origine.
pub fn analyser_ligne(ligne: &str) -> Result<Livre, ErreurMediatheque> {
    let champs: Vec<&str> = ligne.split(';').collect();
    if champs.len() != 5 {
        // [26] exactement 5 champs
        return Err(ligne_invalide(ligne));
    }
    let id: u32 = champs[0]
        .trim()
        .parse()
        .map_err(|_| ligne_invalide(ligne))?;
    let annee = champs[3].trim();
    let annee: u16 = annee.parse().map_err(|_| ligne_invalide(ligne))?; // [10] convertir l'erreur avant ?
    let genre = Genre::depuis_texte(champs[4]).ok_or(ligne_invalide(ligne))?;
    Ok(Livre::new(
        id,
        champs[1].trim(),
        champs[2].trim(),
        annee,
        genre,
    ))
}

/// Construit une médiathèque depuis le contenu d'un fichier.
/// La première ligne (en-tête) et les lignes vides sont ignorées. Les lignes invalides
/// ne bloquent pas le chargement : leurs erreurs sont renvoyées à côté de la médiathèque.
pub fn charger_texte(contenu: &str) -> (Mediatheque, Vec<ErreurMediatheque>) {
    let mut media = Mediatheque::new();
    let mut erreurs = Vec::new();
    for (numero, ligne) in contenu.lines().enumerate() {
        if numero == 0 || ligne.trim().is_empty() {
            // [27] la ligne 0 est l'en-tête
            continue;
        }
        match analyser_ligne(ligne) {
            Ok(livre) => {
                media.ajouter(livre);
            }
            Err(e) => erreurs.push(e),
        }
    }
    (media, erreurs)
}

/// Lit le fichier puis le charge avec `charger_texte`.
/// Erreur Fichier (avec le chemin) si le fichier ne peut pas être lu.
pub fn charger_fichier(
    chemin: &str,
) -> Result<(Mediatheque, Vec<ErreurMediatheque>), ErreurMediatheque> {
    let contenu =
        fs::read_to_string(chemin).map_err(|_| ErreurMediatheque::Fichier(chemin.to_string()))?;
    Ok(charger_texte(&contenu))
}
