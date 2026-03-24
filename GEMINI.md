# Kofferwechsel-Software: Aktueller Fokus

## 🚚 Was ist ein Kofferwechsel?
- **Konzept:** Der wertvolle medizinische Koffer (RTW) wird auf ein neues Fahrgestell ("Chassis") ummontiert ("Hochzeit").
- **Ziel:** Kosten- und Ressourceneinsparung durch Erhalt des teuren Spezialaufbaus.

## 🎯 Hauptziel
KI-unterstützte Auftragsverwaltung für Sonderfahrzeuge.

## 🚀 Roadmap & Perspektive
- **Aktueller Fokus:** Stabilisierung der Kernfunktionen, UI-Feinschliff und Datensicherheit.
- **Perspektivisches Ziel (nach erfolgreicher Testphase):** 
    - **KI-Ähnlichkeitssuche:** Implementierung einer Logik, die basierend auf dem Kunden oder Koffer-Hersteller vergleichbare Altaufträge (auch aus dem Archiv) findet.
    - **Ziel der KI:** Automatisches Vorbereiten der Teileliste basierend auf Dubletten/ähnlichen Projekten, um den Erstellungsprozess zu beschleunigen.

## 📌 Architektur-Leitplanken
- **Backend:** Axum (Port 3000) + SQLite (sqlx). 
- **Frontend:** Yew (WASM, Port 8080).
- **Datumsformat:** Konsequent `TT.MM.JJJJ`.
- **Status-Logik:** Abgeschlossen/Storniert wandern automatisch ins Archiv.
- **UI:** Sidebar-Navigation, absolute API-URLs (`127.0.0.1:3000`), Drag & Drop Upload.

## ⚠️ Datensicherheit
- Die `.db` Datei darf nicht mehr gelöscht werden. Schema-Änderungen müssen über Migrationen oder robuste JSON-Standardwerte gelöst werden.
