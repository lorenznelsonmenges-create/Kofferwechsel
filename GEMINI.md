# Projekt-Gedächtnis: Kofferwechsel-Software (Sonderfahrzeuge)

## 🚚 Was ist ein Kofferwechsel?
- **Konzept:** Der wertvolle medizinische Koffer (RTW) wird auf ein neues Fahrgestell ("Chassis") ummontiert ("Hochzeit").
- **Ziel:** Kosten- und Ressourceneinsparung durch Erhalt des teuren Spezialaufbaus.

## 🎯 Hauptziel
Spezialisierte **Auftragsverwaltung für Kofferwechsel** (ehemals Carsharing-Backend).

## 🛠 Aktueller Stand (März 2026)
1. **Backend:** Persistent via **SQLite** (`sqlx`). Simulation komplett entfernt.
2. **Datenmodell:** Aufträge enthalten Koffer-Daten, Altfahrzeug-Daten, Neufahrzeug-Daten, Teilelisten (mit Produktlinks statt Status) und Bilder.
3. **Archivierung:** Abgeschlossene Aufträge werden in ein separates Archiv verschoben.
4. **API:** Nutzt Axum 0.8 mit Multipart-Upload. Absolute URLs (`http://127.0.0.1:3000`) werden im Frontend bevorzugt.

## 🎨 UI/UX Konventionen (WICHTIG)
- **Name:** Die Software heißt offiziell **"Kofferwechsel-Software"**.
- **Auftragsverwaltung:** 
    - Suche dient als Navigation ("Jump-to-Details") via Auftragsnummer (bündig im Layout, mit "Suche:" Label).
    - Status-Dropdowns sind farblich codiert (Blau, Orange, Lila, Grün, Grau).
    - Mülleimer-Symbol ermöglicht das unwiderrufliche Löschen von Aufträgen (nach Bestätigung).
- **Detailansicht (Spezifikation):**
    - **Kunde & Auftragsnummer** stehen prominent nebeneinander ganz oben.
    - **Teileliste:** Steht ganz oben. Enthält Felder für Bezeichnung, Artikel-Nr, Händler und Produktlink. Teile können einzeln gelöscht werden (Mülleimer).
    - **Bilder:** Werden via **Drag & Drop** hochgeladen.
    - **Umsatzerwartung:** Eigene Karte ganz unten mit großer Anzeige.
- **Formular "Neuer Auftrag":**
    - **Validierung:** Button erst aktiv, wenn alle Felder (inkl. Baujahr/KM) ausgefüllt sind.
    - **Reset:** Alle Felder werden nach erfolgreichem Anlegen automatisch geleert (Controlled Components).
    - **Labels:** Begriffe "Altes Fahrzeug" und "Neues Fahrzeug" verwenden.
    - **Zahlenfelder:** Keine Spin-Buttons (Hoch/Runter-Pfeile) sichtbar.

## 🚀 Roadmap
- **KI-Unterstützung:** Integration einer Logik, die basierend auf Kunden/Koffer vergleichbare Altaufträge findet, um Teilelisten automatisch vorzubereiten.

## 📌 Technische Regeln
- **Typ-Sicherheit:** Strukturen mit `f64` dürfen NIEMALS `Eq` ableiten (nur `PartialEq`).
- **Build-Prozess:** Frontend via `trunk build`, Backend via `cargo run`.
- **Datenintegrität:** Bei Schema-Änderungen in den Structs muss die `.db` Datei im Backend gelöscht werden, um JSON-Fehler zu vermeiden.
