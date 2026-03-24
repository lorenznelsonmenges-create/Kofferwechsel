use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AuftragsStatus {
    Angenommen,
    InArbeit,
    Bereitstellung,
    Abgeschlossen,
    Archiviert,
    Storniert,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BestellStatus {
    Offen,
    Bestellt,
    Geliefert,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Bestellteil {
    pub name: String,
    pub artikel_nummer: String,
    pub haendler: String,
    pub status: BestellStatus,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Auftraggeber {
    pub name: String,
    pub kontakt: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Koffer {
    pub seriennummer: String,
    pub hersteller: String,
    pub baujahr: u32,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Fahrgestell {
    pub vin: String,
    pub kennzeichen: String,
    pub modell: String,
    pub kilometerstand: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct KofferwechselAuftrag {
    pub auftrags_nummer: String,
    pub status: AuftragsStatus,
    pub auftraggeber: Auftraggeber,
    pub koffer: Koffer,
    pub spender_fahrgestell: Fahrgestell,
    pub empfaenger_fahrgestell: Fahrgestell,
    pub start_datum: String,
    pub geplante_hochzeit: String,
    pub abschluss_datum: Option<String>,
    pub umsatz: f64,
    pub arbeitsstunden: f64,
    pub bilder: Vec<String>,
    pub checkliste: std::collections::HashMap<String, bool>,
    pub teileliste: Vec<Bestellteil>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KofferManagement {
    pub auftraege: Vec<KofferwechselAuftrag>,
    pub kunden: Vec<Auftraggeber>,
}

pub trait KofferService {
    fn erstelle_auftrag(&mut self, auftrag: KofferwechselAuftrag) -> bool;
    fn aktualisiere_status(&mut self, auftrags_nummer: &str, neuer_status: AuftragsStatus) -> bool;
    fn get_auftrag(&self, auftrags_nummer: &str) -> Option<KofferwechselAuftrag>;
    fn lade_aktive_auftraege(&self) -> Vec<KofferwechselAuftrag>;
    fn archiviere_auftrag(&mut self, auftrags_nummer: &str) -> bool;
    fn teil_hinzufuegen(&mut self, auftrags_nummer: &str, teil: Bestellteil) -> bool;
}

impl KofferService for KofferManagement {
    fn erstelle_auftrag(&mut self, auftrag: KofferwechselAuftrag) -> bool {
        if self.auftraege.iter().any(|a| a.auftrags_nummer == auftrag.auftrags_nummer) {
            return false;
        }
        self.auftraege.push(auftrag);
        true
    }

    fn aktualisiere_status(&mut self, auftrags_nummer: &str, neuer_status: AuftragsStatus) -> bool {
        if let Some(auftrag) = self.auftraege.iter_mut().find(|a| a.auftrags_nummer == auftrags_nummer) {
            auftrag.status = neuer_status.clone();
            if neuer_status == AuftragsStatus::Abgeschlossen {
                auftrag.abschluss_datum = Some("2024-03-24".to_string());
            }
            true
        } else {
            false
        }
    }

    fn get_auftrag(&self, auftrags_nummer: &str) -> Option<KofferwechselAuftrag> {
        self.auftraege.iter()
            .find(|a| a.auftrags_nummer == auftrags_nummer)
            .cloned()
    }

    fn lade_aktive_auftraege(&self) -> Vec<KofferwechselAuftrag> {
        self.auftraege.iter()
            .filter(|a| a.status != AuftragsStatus::Archiviert && a.status != AuftragsStatus::Storniert)
            .cloned()
            .collect()
    }

    fn archiviere_auftrag(&mut self, auftrags_nummer: &str) -> bool {
        if let Some(auftrag) = self.auftraege.iter_mut().find(|a| a.auftrags_nummer == auftrags_nummer) {
            if auftrag.status == AuftragsStatus::Abgeschlossen {
                auftrag.status = AuftragsStatus::Archiviert;
                return true;
            }
        }
        false
    }

    fn teil_hinzufuegen(&mut self, auftrags_nummer: &str, teil: Bestellteil) -> bool {
        if let Some(auftrag) = self.auftraege.iter_mut().find(|a| a.auftrags_nummer == auftrags_nummer) {
            auftrag.teileliste.push(teil);
            return true;
        }
        false
    }
}

impl KofferManagement {
    pub fn new() -> Self {
        Self {
            auftraege: vec![],
            kunden: vec![],
        }
    }
}
