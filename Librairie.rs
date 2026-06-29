#[derive(Debug, Clone, PartialEq)]
pub enum Statut {
    Disponible,
    Emprunte,
}

pub trait Affichable {
    fn vers_chaine(&self) -> String;
}

impl Affichable for Statut {
    fn vers_chaine(&self) -> String {
        match self {
            Statut::Disponible => String::from("Disponible"),
            Statut::Emprunte => String::from("Emprunté"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    pub annee: u32,
    pub statut: Statut,
}
