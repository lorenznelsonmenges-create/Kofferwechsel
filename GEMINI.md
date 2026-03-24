# Projekt-Gedächtnis: Kofferwechsel-Software (Sonderfahrzeuge)

## 🚚 Was ist ein Kofferwechsel?
- **Konzept:** Der wertvolle medizinische Koffer (RTW) wird auf ein neues Fahrgestell ("Chassis") ummontiert ("Hochzeit").
- **Ziel:** Kosten- und Ressourceneinsparung durch Erhalt des teuren Spezialaufbaus.

## 🎯 Hauptziel
Spezialisierte **Auftragsverwaltung für Kofferwechsel** (ehemals Carsharing-Backend).

## 🛠 Aktueller Stand (März 2026)
1. **Backend:** Persistent via **SQLite** (`sqlx`). API-Endpunkte für State-Management, Image-Upload und Bild-Löschung (`/api/delete-image`).
2. **Datenmodell:** Aufträge enthalten Koffer-Daten, Altfahrzeug-Daten, Neufahrzeug-Daten, Teilelisten (mit Produktlinks) und Bilder.
3. **Status-Logik:** Die Status-Optionen sind: `Angenommen`, `InArbeit`, `Bereitstellung`, `Abgeschlossen` und `Storniert`.
4. **Archivierung:** Aufträge mit Status `Abgeschlossen` oder `Storniert` werden automatisch im Archiv angezeigt und aus der aktiven Auftragsverwaltung entfernt. Reaktivierung über Status-Dropdown in den Details möglich.
5. **API:** Nutzt Axum 0.8. Absolute URLs (`http://127.0.0.1:3000`) werden im Frontend bevorzugt.

## 🎨 UI/UX Konventionen (WICHTIG)
- **Name:** Die Software heißt offiziell **"Kofferwechsel-Software"**.
- **Sidebar:** Der Software-Name oben links führt per Klick zurück zum Dashboard.
- **Auftragsverwaltung:** 
    - Suche dient als Navigation ("Jump-to-Details") via Auftragsnummer (nur Nummern-Vorschläge).
    - Status-Dropdowns sind farblich codiert (Blau, Orange, Lila, Grün, Rot).
    - Dropdowns haben einen visuellen Pfeil-Indikator (dynamisches SVG).
    - Mülleimer-Symbol ermöglicht das Löschen von Aufträgen (mit Confirm-Dialog).
    - Hinweistext erscheint, wenn keine aktiven Aufträge vorhanden sind.
- **Detailansicht (Spezifikation):**
    - **Kunde & Auftragsnummer** stehen prominent nebeneinander ganz oben.
    - **Teileliste:** Steht ganz oben. Enthält Bezeichnung, Artikel-Nr, Händler und Produktlink (Icon öffnet neuen Tab). Teile können gelöscht werden.
    - **Bilder & Dokumente:** Werden via **Drag & Drop** hochgeladen. Anzeige als schlanke Liste mit Dateinamen.
    - **Vorschau:** Klick auf Dateiname öffnet Lightbox-Vorschau. Speicher-Icon öffnet Datei in neuem Tab.
    - **Umsatzerwartung:** Separate Karte ganz unten.
- **Formular "Neuer Auftrag":**
    - **Validierung:** Button erst aktiv, wenn alle Felder (inkl. Baujahr/KM) explizit ausgefüllt sind (0 ist erlaubt).
    - **Eindeutigkeit:** Auftragsnummern werden auf Dubletten geprüft (Warnung + Sperre).
    - **Reset:** Felder werden nach Anlegen automatisch geleert (Controlled Components).
    - **Labels:** Begriffe "Altes Fahrzeug" und "Neues Fahrzeug" verwenden.
    - **Zahlenfelder:** Keine Spin-Buttons (Hoch/Runter-Pfeile) sichtbar.

## 🚀 Roadmap
- **KI-Unterstützung:** Integration einer Logik, die basierend auf Kunden/Koffer vergleichbare Altaufträge findet, um Teilelisten automatisch vorzubereiten.

## 📌 Technische Regeln & Datensicherheit
- **Typ-Sicherheit:** Strukturen mit `f64` dürfen NIEMALS `Eq` ableiten (nur `PartialEq`).
- **Build-Prozess:** Frontend via `trunk build`, Backend via `cargo run`.
- **Datenintegrität (KRITISCH):** 
    - In der Produktion darf die `.db` NIEMALS gelöscht werden. 
    - Schema-Änderungen im Rust-Code (Enums/Structs) führen zu Deserialisierungsfehlern in alten DB-Beständen -> Zukünftig SQLx-Migrationen nutzen.
    - Backups: Backend Snapshots einplanen.
