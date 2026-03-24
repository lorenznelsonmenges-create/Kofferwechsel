# Projekt-Gedächtnis: Kofferwechsel-Software (Sonderfahrzeuge)

## 🚚 Was ist ein Kofferwechsel?
- **Konzept:** Der wertvolle medizinische Koffer (RTW) wird auf ein neues Fahrgestell ("Chassis") ummontiert ("Hochzeit").
- **Ziel:** Kosten- und Ressourceneinsparung durch Erhalt des teuren Spezialaufbaus.

## 🎯 Hauptziel
Spezialisierte **Auftragsverwaltung für Kofferwechsel** (ehemals Carsharing-Backend).

## 🛠 Aktueller Stand (März 2026)
1. **Backend:** Persistent via **SQLite** (`sqlx`). API-Endpunkte für State-Management, Image-Upload und Bild-Löschung (`/api/delete-image`).
2. **Datenmodell:** Aufträge enthalten Koffer-Daten, Altfahrzeug-Daten, Neufahrzeug-Daten, Teilelisten (mit Produktlinks) und Bilder.
3. **Archivierung:** Abgeschlossene Aufträge werden in ein separates Archiv verschoben. "Zurück"-Buttons navigieren kontextsensitiv (Archiv vs. Auftragsverwaltung).
4. **API:** Nutzt Axum 0.8. Absolute URLs (`http://127.0.0.1:3000`) werden im Frontend bevorzugt.

## 🎨 UI/UX Konventionen (WICHTIG)
- **Name:** Die Software heißt offiziell **"Kofferwechsel-Software"**.
- **Auftragsverwaltung:** 
    - Suche dient als Navigation ("Jump-to-Details") via Auftragsnummer (nur Nummern-Vorschläge).
    - Status-Dropdowns sind farblich codiert (Blau, Orange, Lila, Grün, Grau).
    - Mülleimer-Symbol ermöglicht das Löschen von Aufträgen (mit Confirm-Dialog).
- **Detailansicht (Spezifikation):**
    - **Kunde & Auftragsnummer** stehen prominent nebeneinander ganz oben.
    - **Teileliste:** Steht ganz oben. Enthält Bezeichnung, Artikel-Nr, Händler und Produktlink (Icon öffnet neuen Tab).
    - **Bilder & Dokumente:** Werden via **Drag & Drop** hochgeladen. Anzeige als Liste mit Dateinamen.
    - **Vorschau:** Klick auf Dateiname öffnet Lightbox-Vorschau. Speicher-Icon öffnet Datei in neuem Tab.
    - **Umsatzerwartung:** Separate Karte ganz unten.
- **Formular "Neuer Auftrag":**
    - **Validierung:** Button erst aktiv, wenn alle Felder (inkl. Baujahr/KM) explizit ausgefüllt sind (0 ist erlaubt).
    - **Reset:** Felder werden nach Anlegen automatisch geleert (Controlled Components).
    - **Labels:** "Altes Fahrzeug" und "Neues Fahrzeug". Keine Spin-Buttons in Zahlenfeldern.

## 🚀 Roadmap
- **KI-Unterstützung:** Integration einer Logik, die basierend auf Kunden/Koffer vergleichbare Altaufträge findet, um Teilelisten automatisch vorzubereiten.

## 📌 Technische Regeln
- **Typ-Sicherheit:** Strukturen mit `f64` dürfen NIEMALS `Eq` ableiten (nur `PartialEq`).
- **Build-Prozess:** Frontend via `trunk build`, Backend via `cargo run`.
- **Datenintegrität:** Bei Schema-Änderungen in den Structs muss die `.db` Datei im Backend gelöscht werden.
