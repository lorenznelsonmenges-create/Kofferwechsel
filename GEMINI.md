# Projekt-Gedächtnis: Kofferwechsel (Sonderfahrzeuge)

## 🚚 Was ist ein Kofferwechsel?
- **Fahrzeugtyp:** Fokus liegt auf **Rettungswägen (RTW)**.
- **Konzept:** Der Koffer (der medizinische Spezialaufbau) ist wesentlich langlebiger und teurer als das darunterliegende Fahrgestell (Chassis).
- **Prozess:** Ähnlich einer "Organtransplantation" wird der wertvolle Koffer vom alten, verschlissenen Fahrgestell abgenommen, bei Bedarf generalüberholt und auf ein fabrikneues Chassis montiert ("Hochzeit").
- **Ziel:** Massive Kosten- und Ressourceneinsparung, da nur das Verschleißteil (Auto) ersetzt wird, während das Kernstück (der Koffer) erhalten bleibt.

## 🎯 Hauptziel
Umbau des bestehenden Carsharing-Backends zu einer spezialisierten **Auftragsverwaltung für Kofferwechsel bei Sonderfahrzeugen**.

## 🛠 Aktueller Fokus & Architektur
1. **Entfernung der Simulation:** Das System soll von einem simulierten Verhalten auf ein reales, persistentes Programm umgestellt werden.
2. **Backend-Refactoring:** Umstellung der Logik von "Fahrzeugbuchung" auf "Auftragsmanagement" (Kofferwechsel).
3. **Deployment-Strategie:** 
   - Phase 1: Deployment auf **GitPod** (für umfangreiche Tests).
   - Phase 2: Rollout auf **Kunden-Server** mit dedizierter Datenbank.
4. **Datenhaltung:** Implementierung einer Archivierungsfunktion für abgeschlossene Aufträge.

## 🚀 Zukunfts-Features (Roadmap)
- **KI-Unterstützung:** Integration einer KI, die vergleichbare Altaufträge findet, um Bestelllisten (Teile) bei Dubletten automatisch zu generieren.

## 📋 Fachlogik & Prozess-Details (Fahrtec Fokus)
- **System-Check:** Unterscheidung zwischen analogen Schaltersystemen und modernen CAN-Bus-Systemen (entscheidend für Material/Stunden).
- **Verschleißteil-Standard:** Jedes Projekt umfasst standardmäßig die Prüfung/Erneuerung von:
    - Schlosstechnik (Heck-, Seiten-, Zusatztüren).
    - Magnetfeststeller (270 Grad) und Sturmhaken.
    - V2A Stangenscharniere.
    - Silikonfugen (außen).
- **Komponenten-Module:** 
    - **Klima/Heizung:** Prüfung Leitungen, Kondensator, Standheizung.
    - **Medizintechnik:** Tragentisch-Überholung, Sauerstoffversorgungs-Prüfung.
    - **Elektrik:** 230V Wandler, Ladetechnik, Blaulicht/SSA, Unfalldatenspeicher (UDS).
- **Fahrzeug-Bereitstellung (Neu-Chassis):** Da Spender-Fahrzeuge oft Unfallschäden haben, liegt der Fokus auf der technischen Abnahme des Neufahrzeugs nach dem Umbau:
    - Umbau/Anpassung Auspuffanlage.
    - Transfer/Einbau Retarder (falls vorhanden).
    - Montage Schmutzfänger und Außenbeklebung.
    - Herstellung der vollen Gebrauchsfertigkeit (Klima, Heizung, Trennwandfenster).
- **Qualitätssicherung:** Abschlussprüfung nach VDE 0100, Sauerstoff-Druckprobe und Ergänzung der Zulassungsbescheinigung.

## 📌 Wichtige Regeln
- Keine Simulation mehr nutzen.
- Fokus auf Datenintegrität (Archivierung).
- Code modular halten für den späteren KI-Anschluss.
- **Teilebestellungen:** Bestellungen sind **strikt auftragsgebunden**.
- **Typ-Sicherheit:** Strukturen mit `f64` (Umsatz, Stunden) dürfen NIEMALS `Eq` ableiten (nur `PartialEq`).
- **UI-Design:** Keine Abkürzungen (z.B. "Kennzeichen" statt "KZ", "Kilometerstand" statt "KM", "Seriennummer" statt "SN").
