# Projekt-Gedächtnis: Kofferwechsel-Software (Sonderfahrzeuge)

## 🚚 Was ist ein Kofferwechsel?
- **Konzept:** Der wertvolle medizinische Koffer (RTW) wird auf ein neues Fahrgestell ("Chassis") ummontiert ("Hochzeit").
- **Ziel:** Kosten- und Ressourceneinsparung durch Erhalt des teuren Spezialaufbaus.

## 🎯 Hauptziel
Spezialisierte **Auftragsverwaltung für Kofferwechsel** (ehemals Carsharing-Backend).

## 🛠 Aktueller Stand (März 2026)
1. **Backend:** Persistent via **SQLite** (`sqlx`). API-Endpunkte für State, Image-Upload und Löschung.
2. **Datenmodell:** Aufträge enthalten Koffer-Daten, Altfahrzeug-Daten, Neufahrzeug-Daten, Teilelisten und Bilder.
3. **Status-Logik:** `Angenommen`, `InArbeit`, `Bereitstellung` (aktiv) sowie `Abgeschlossen` und `Storniert` (Archiv).
4. **Automatisierung:** 
    - **Daten:** Startdatum wird bei Erstellung automatisch erfasst (editierbar). Abschlussdatum wird bei Statusänderung auf "Abgeschlossen" automatisch gesetzt.
    - **Format:** Datumsangaben konsequent in **TT.MM.JJJJ**.
5. **Archivierung:** Automatischer Transfer ins Archiv bei Abschluss/Stornierung. Reaktivierung über Status-Dropdown möglich.

## 🎨 UI/UX Konventionen (WICHTIG)
- **Name:** Die Software heißt offiziell **"Kofferwechsel-Software"**.
- **Auftragsverwaltung:** 
    - Suche via Auftragsnummer (nur Nummern-Vorschläge).
    - Status-Dropdowns farblich codiert (Blau, Orange, Lila, Grün, Rot) mit Pfeil-Indikator.
    - Info-Texte erscheinen, wenn Listen leer sind ("Keine aktiven Aufträge vorhanden").
- **Detailansicht (Spezifikation):**
    - Kunde & Auftragsnummer prominent oben.
    - Teileliste (Bezeichnung, Art-Nr, Händler, Produktlink) inkl. Löschfunktion.
    - Dokumentenliste mit Lightbox-Vorschau und "In neuem Tab öffnen"-Funktion.
- **Formular "Neuer Auftrag":**
    - Validierung: Alle Felder inkl. KM und Baujahr sind Pflicht (0 erlaubt).
    - Dublettenprüfung für Auftragsnummern.

## 🚀 Roadmap
- **Nächster Schritt:** Anpassung der **Zeilenbeschriftungen in der Detailansicht**, um die Lesbarkeit der technischen Daten zu erhöhen.
- **KI-Unterstützung:** Integration einer Logik zur Suche vergleichbarer Altaufträge.

## 📌 Technische Regeln & Datensicherheit
- **Typ-Sicherheit:** Strukturen mit `f64` dürfen NIEMALS `Eq` ableiten (nur `PartialEq`).
- **Datenintegrität:** In Produktion `.db` niemals löschen. Zukünftig SQLx-Migrationen für Schema-Änderungen nutzen.
