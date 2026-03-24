use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AuftragsStatus {
    Angenommen,
    InArbeit,
    Bereitstellung,
    Abgeschlossen,
    Storniert,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Bestellteil {
    pub name: String,
    pub artikel_nummer: String,
    pub haendler: String,
    pub produkt_link: Option<String>,
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

impl KofferManagement {
    pub fn new() -> Self {
        Self {
            auftraege: vec![],
            kunden: vec![],
        }
    }
}
