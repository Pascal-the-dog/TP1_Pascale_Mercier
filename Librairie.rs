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

impl Description for Livre {
    fn obtenir_resume(&self) -> String {
        let statut_str = match self.statut {
            Statut::Disponible => "Disponible",
            Statut::Emprunte => "Emprunté",
        };
        format!("'{}' par {} ({}) - [Statut: {}]", self.titre, self.auteur, self.annee, statut_str)
    }
}
