use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, File};
use std::collections::HashMap;
use rust_frontend::kofferwechsel::{KofferManagement, KofferwechselAuftrag, AuftragsStatus, Auftraggeber, Koffer, Fahrgestell, Bestellteil};

#[derive(Clone, PartialEq)]
enum Tab { Dashboard, Cockpit, NeuerAuftrag, Details(String), Archiv }

fn get_current_date() -> String {
    let now = js_sys::Date::new_0();
    let d = now.get_date() as u32;
    let m = (now.get_month() as u32) + 1;
    let y = now.get_full_year() as u32;
    format!("{:02}.{:02}.{}", d, m, y)
}

#[function_component(App)]
fn app() -> Html {
    let tab = use_state(|| Tab::Dashboard);
    let management = use_state(KofferManagement::new);
    let search = use_state(|| String::new());
    let info = use_state(|| String::new());
    let selected_image = use_state(|| None::<String>);

    let f_nr = use_state(|| String::new()); let f_kn = use_state(|| String::new());
    let f_ks = use_state(|| String::new()); let f_kh = use_state(|| String::new()); let f_kj = use_state(|| None::<u32>);
    let f_sv = use_state(|| String::new()); let f_skz = use_state(|| String::new()); let f_skm = use_state(|| None::<u32>);
    let f_ev = use_state(|| String::new()); let f_ekz = use_state(|| String::new()); let f_ekm = use_state(|| None::<u32>);
    let f_ums = use_state(|| 0.0f64);
    let f_date = use_state(get_current_date);

    let load = {
        let m = management.clone();
        Callback::from(move |_| {
            let m = m.clone();
            spawn_local(async move {
                if let Ok(resp) = Request::get("http://127.0.0.1:3000/api/state").send().await {
                    if let Ok(data) = resp.json::<KofferManagement>().await { m.set(data); }
                }
            });
        })
    };
    { let l = load.clone(); use_effect_with((), move |_| { l.emit(()); || () }); }

    let save = {
        let m = management.clone(); let i = info.clone();
        Callback::from(move |nm: KofferManagement| {
            let m = m.clone(); let i = i.clone();
            spawn_local(async move {
                let req = Request::post("http://127.0.0.1:3000/api/state").json(&nm);
                if let Ok(r) = req { if let Ok(_) = r.send().await { m.set(nm); i.set("Gespeichert.".to_string()); } }
            });
        })
    };

    let on_create = {
        let m = management.clone(); let s = save.clone(); let t = tab.clone();
        let fnr = f_nr.clone(); let fkn = f_kn.clone(); let fks = f_ks.clone(); let fkh = f_kh.clone(); let fkj = f_kj.clone();
        let fsv = f_sv.clone(); let fskz = f_skz.clone(); let fskm = f_skm.clone();
        let fev = f_ev.clone(); let fekz = f_ekz.clone(); let fekm = f_ekm.clone(); let fums = f_ums.clone();
        let fdate = f_date.clone();
        Callback::from(move |_| {
            let mut nm = (*m).clone();
            let mut cl = HashMap::new();
            for i in ["Auspuff", "Retarder", "Schmutzfänger", "Beklebung", "VDE 0100", "O2-Probe"] { cl.insert(i.to_string(), false); }
            nm.auftraege.push(KofferwechselAuftrag {
                auftrags_nummer: (*fnr).clone(), status: AuftragsStatus::Angenommen, auftraggeber: Auftraggeber { name: (*fkn).clone(), kontakt: "".to_string() },
                koffer: Koffer { seriennummer: (*fks).clone(), hersteller: (*fkh).clone(), baujahr: fkj.unwrap_or(2024) },
                spender_fahrgestell: Fahrgestell { vin: (*fsv).clone(), kennzeichen: (*fskz).clone(), modell: "RTW".to_string(), kilometerstand: fskm.unwrap_or(0) },
                empfaenger_fahrgestell: Fahrgestell { vin: (*fev).clone(), kennzeichen: (*fekz).clone(), modell: "RTW".to_string(), kilometerstand: fekm.unwrap_or(0) },
                start_datum: (*fdate).clone(), geplante_hochzeit: "".to_string(), abschluss_datum: None, umsatz: *fums, arbeitsstunden: 0.0, bilder: vec![], teileliste: vec![], checkliste: cl,
            });
            s.emit(nm); 
            fnr.set(String::new()); fkn.set(String::new()); fks.set(String::new()); fkh.set(String::new()); fkj.set(None); fsv.set(String::new()); fskz.set(String::new()); fskm.set(None); fev.set(String::new()); fekz.set(String::new()); fekm.set(None); fums.set(0.0);
            fdate.set(get_current_date());
            t.set(Tab::Cockpit);
        })
    };

    let card = "background:#fff; border-radius:16px; padding:24px; box-shadow:0 4px 20px rgba(0,0,0,0.05); margin-bottom:24px; box-sizing:border-box;";
    let inp = "padding:10px; border:1px solid #dee2e6; border-radius:10px; width:100%; margin-bottom:10px; font-size:14px; box-sizing:border-box;";

    let content = match &*tab {
        Tab::Dashboard => {
            let ab = management.auftraege.iter().filter(|a| a.status == AuftragsStatus::Abgeschlossen).collect::<Vec<_>>();
            let ak = management.auftraege.iter().filter(|a| a.status != AuftragsStatus::Abgeschlossen && a.status != AuftragsStatus::Storniert).collect::<Vec<_>>();
            let gu: f64 = ab.iter().map(|a| a.umsatz).sum();
            let pu: f64 = ak.iter().map(|a| a.umsatz).sum();
            let mut vorschlaege: Vec<String> = management.auftraege.iter().map(|a| a.auftrags_nummer.clone()).collect();
            vorschlaege.sort(); vorschlaege.dedup();
            html! {
                <div>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:32px;">
                        <h1>{"Dashboard"}</h1>
                        <div style="display:flex; align-items:center; gap:12px;">
                            <span style="font-weight:700; color:#adb5bd; font-size:12px; text-transform:uppercase;">{"Suche:"}</span>
                            <input style="width:250px; padding:10px 16px; border-radius:12px; border:1px solid #e9ecef; font-size:14px; outline:none; box-sizing:border-box;" list="dashboard-vorschlaege" placeholder="Auftragsnummer..." value={(*search).clone()} oninput={let s=search.clone(); let t=tab.clone(); let m=management.clone(); move |e: InputEvent| { let val=e.target_unchecked_into::<HtmlInputElement>().value(); s.set(val.clone()); if let Some(a)=m.auftraege.iter().find(|a| a.auftrags_nummer==val) { t.set(Tab::Details(a.auftrags_nummer.clone())); s.set("".to_string()); } }} />
                            <datalist id="dashboard-vorschlaege">{ for vorschlaege.iter().map(|v| html! { <option value={v.clone()} /> }) }</datalist>
                        </div>
                    </div>
                    <div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:24px;">
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"UMSATZ (FERTIG)"}</div><div style="font-size:42px; font-weight:800; color:#006666;">{format!("{:.0} €", gu.abs())}</div></div>
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"PIPELINE (AKTIV)"}</div><div style="font-size:42px; font-weight:800; color:#fd7e14;">{format!("{:.0} €", pu.abs())}</div></div>
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"Ø PRO AUFTRAG"}</div><div style="font-size:42px; font-weight:800; color:#228be6;">{format!("{:.0} €", (if ab.is_empty() {0.0} else {gu/ab.len() as f64}).abs())}</div></div>
                    </div>
                </div>
            }
        },
        Tab::Cockpit => {
            let gef = management.auftraege.iter().filter(|a| a.status != AuftragsStatus::Abgeschlossen && a.status != AuftragsStatus::Storniert).collect::<Vec<_>>();
            let mut vorschlaege: Vec<String> = management.auftraege.iter().map(|a| a.auftrags_nummer.clone()).collect();
            vorschlaege.sort(); vorschlaege.dedup();
            html! {
                <div>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:32px;">
                        <h1 style="margin:0;">{"🚗 Auftragsverwaltung"}</h1>
                        <div style="display:flex; align-items:center; gap:12px;">
                            <span style="font-weight:700; color:#adb5bd; font-size:12px; text-transform:uppercase;">{"Suche:"}</span>
                            <input style="width:250px; padding:10px 16px; border-radius:12px; border:1px solid #e9ecef; font-size:14px; outline:none; box-sizing:border-box;" list="search-vorschlaege" placeholder="Auftragsnummer..." value={(*search).clone()} oninput={let s=search.clone(); let t=tab.clone(); let m=management.clone(); move |e: InputEvent| { let val=e.target_unchecked_into::<HtmlInputElement>().value(); s.set(val.clone()); if let Some(a)=m.auftraege.iter().find(|a| a.auftrags_nummer==val) { t.set(Tab::Details(a.auftrags_nummer.clone())); s.set("".to_string()); } }} />
                            <datalist id="search-vorschlaege">{ for vorschlaege.iter().map(|v| html! { <option value={v.clone()} /> }) }</datalist>
                        </div>
                    </div>
                    { if gef.is_empty() { html! { <div style={card}>{"Keine aktiven Aufträge vorhanden."}</div> } } else {
                        html! { <> { for gef.iter().map(|a| {
                            let nr = a.auftrags_nummer.clone(); let t = tab.clone(); let ms = management.clone(); let ss = save.clone();
                            let (bg, color) = match a.status { AuftragsStatus::Angenommen => ("#e7f5ff", "#228be6"), AuftragsStatus::InArbeit => ("#fff4e6", "#fd7e14"), AuftragsStatus::Bereitstellung => ("#f3f0ff", "#7950f2"), AuftragsStatus::Abgeschlossen => ("#ebfbee", "#40c057"), AuftragsStatus::Storniert => ("#fff5f5", "#fa5252") };
                            html! {
                                <div style={card}>
                                    <div style="display:flex; justify-content:space-between;">
                                        <div><div style="font-size:12px; color:#666;">{&a.auftrags_nummer}{" · Erstellt: "}{&a.start_datum}</div><h2>{&a.auftraggeber.name}</h2></div>
                                        <div style="text-align:right;">
                                            <div style="font-size:20px; font-weight:800; color:#006666; margin-bottom:8px;">{format!("{:.0} €", a.umsatz)}</div>
                                            <select style={format!("padding:6px 28px 6px 12px; border-radius:12px; border:none; font-size:12px; font-weight:700; text-transform:uppercase; cursor:pointer; background:{bg}; color:{color}; appearance:none; -webkit-appearance:none; text-align:center; min-width:140px; background-image: url(\"data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='{}'%3e%3cpath d='M7 10l5 5 5-5z'/%3e%3c/svg%3e\"); background-repeat: no-repeat; background-position: right 8px center; background-size: 18px;", color.replace("#", "%23"))} onchange={let nr=nr.clone(); let m=ms.clone(); let s=ss.clone(); move |e: Event| { let val=e.target_unchecked_into::<web_sys::HtmlSelectElement>().value(); let status=match val.as_str() { "Angenommen"=>AuftragsStatus::Angenommen, "InArbeit"=>AuftragsStatus::InArbeit, "Bereitstellung"=>AuftragsStatus::Bereitstellung, "Abgeschlossen"=>AuftragsStatus::Abgeschlossen, "Storniert"=>AuftragsStatus::Storniert, _=>AuftragsStatus::Angenommen }; let mut nm=(*m).clone(); if let Some(x)=nm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.status=status.clone(); if status == AuftragsStatus::Abgeschlossen { x.abschluss_datum = Some(get_current_date()); } else { x.abschluss_datum = None; } s.emit(nm); } }}>
                                                <option value="Angenommen" selected={a.status == AuftragsStatus::Angenommen}>{"Angenommen"}</option><option value="InArbeit" selected={a.status == AuftragsStatus::InArbeit}>{"In Arbeit"}</option><option value="Bereitstellung" selected={a.status == AuftragsStatus::Bereitstellung}>{"Bereitstellung"}</option><option value="Abgeschlossen" selected={a.status == AuftragsStatus::Abgeschlossen}>{"Abgeschlossen"}</option><option value="Storniert" selected={a.status == AuftragsStatus::Storniert}>{"Storniert"}</option>
                                            </select>
                                        </div>
                                    </div>
                                    <div style="display:grid; grid-template-columns:repeat(3, 1fr); gap:20px; font-size:14px; border-top:1px solid #f1f3f5; margin-top:16px; padding-top:16px;">
                                        <div><div style="font-size:11px; color:#adb5bd;">{"KOFFER-AUFBAU"}</div><div>{format!("{} (SN: {})", a.koffer.hersteller, a.koffer.seriennummer)}</div></div>
                                        <div><div style="font-size:11px; color:#adb5bd;">{"ALTES FAHRZEUG"}</div><div>{format!("{} ({} km)", a.spender_fahrgestell.kennzeichen, a.spender_fahrgestell.kilometerstand)}</div></div>
                                        <div><div style="font-size:11px; color:#adb5bd;">{"NEUES FAHRZEUG"}</div><div>{format!("{} ({} km)", a.empfaenger_fahrgestell.kennzeichen, a.empfaenger_fahrgestell.kilometerstand)}</div></div>
                                    </div>
                                    <div style="margin-top:20px; display:flex; gap:8px; justify-content:space-between; align-items:center;">
                                        <button style="padding:8px 16px; background:#006666; color:#fff; border:none; border-radius:8px; cursor:pointer;" onclick={let t=t.clone(); let n=nr.clone(); move |_| t.set(Tab::Details(n.clone()))}>{"Spezifizieren"}</button>
                                        <button style="background:none; border:none; cursor:pointer; font-size:18px; padding:8px;" onclick={let nr=nr.clone(); let m=ms.clone(); let s=ss.clone(); move |_| { if web_sys::window().unwrap().confirm_with_message(&format!("Auftrag {} löschen?", nr)).unwrap_or(false) { let mut nm=(*m).clone(); nm.auftraege.retain(|x| x.auftrags_nummer!=nr); s.emit(nm); } }}>{"🗑️"}</button>
                                    </div>
                                </div>
                            }
                        }) } </> }
                    }}
                </div>
            }
        },
        Tab::Details(nr) => {
            let a = management.auftraege.iter().find(|a| &a.auftrags_nummer == nr);
            match a {
                Some(a) => {
                    let is_archiv = a.status == AuftragsStatus::Abgeschlossen || a.status == AuftragsStatus::Storniert;
                    let nr_str = a.auftrags_nummer.clone();
                    let (bg, color) = match a.status { AuftragsStatus::Angenommen => ("#e7f5ff", "#228be6"), AuftragsStatus::InArbeit => ("#fff4e6", "#fd7e14"), AuftragsStatus::Bereitstellung => ("#f3f0ff", "#7950f2"), AuftragsStatus::Abgeschlossen => ("#ebfbee", "#40c057"), AuftragsStatus::Storniert => ("#fff5f5", "#fa5252") };
                    html! {
                        <div>
                            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;">
                                <button style="padding:8px 16px; background:#006666; color:#fff; border:none; border-radius:8px; cursor:pointer; font-weight:600;" onclick={let t=tab.clone(); move |_| t.set(if is_archiv { Tab::Archiv } else { Tab::Cockpit })}>{"← Zurück"}</button>
                                <div style="display:flex; align-items:center; gap:16px;">
                                    { if let Some(ende) = &a.abschluss_datum { html! { <span style="font-size:12px; font-weight:700; color:#40c057;">{"Abgeschlossen am: "}{ende}</span> } } else { html! {} } }
                                    <select style={format!("padding:6px 28px 6px 12px; border-radius:12px; border:none; font-size:12px; font-weight:700; text-transform:uppercase; cursor:pointer; background:{bg}; color:{color}; appearance:none; -webkit-appearance:none; text-align:center; min-width:140px; background-image: url(\"data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='{}'%3e%3cpath d='M7 10l5 5 5-5z'/%3e%3c/svg%3e\"); background-repeat: no-repeat; background-position: right 8px center; background-size: 18px;", color.replace("#", "%23"))} onchange={let nr=nr_str.clone(); let m=management.clone(); let s=save.clone(); move |e: Event| { let val=e.target_unchecked_into::<web_sys::HtmlSelectElement>().value(); let status=match val.as_str() { "Angenommen"=>AuftragsStatus::Angenommen, "InArbeit"=>AuftragsStatus::InArbeit, "Bereitstellung"=>AuftragsStatus::Bereitstellung, "Abgeschlossen"=>AuftragsStatus::Abgeschlossen, "Storniert"=>AuftragsStatus::Storniert, _=>AuftragsStatus::Angenommen }; let mut nm=(*m).clone(); if let Some(x)=nm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.status=status.clone(); if status == AuftragsStatus::Abgeschlossen { x.abschluss_datum = Some(get_current_date()); } else { x.abschluss_datum = None; } s.emit(nm); } }}>
                                        <option value="Angenommen" selected={a.status == AuftragsStatus::Angenommen}>{"Angenommen"}</option><option value="InArbeit" selected={a.status == AuftragsStatus::InArbeit}>{"In Arbeit"}</option><option value="Bereitstellung" selected={a.status == AuftragsStatus::Bereitstellung}>{"Bereitstellung"}</option><option value="Abgeschlossen" selected={a.status == AuftragsStatus::Abgeschlossen}>{"Abgeschlossen"}</option><option value="Storniert" selected={a.status == AuftragsStatus::Storniert}>{"Storniert"}</option>
                                    </select>
                                </div>
                            </div>
                            <div style={card}>
                                <div style="margin-bottom:24px; padding-bottom:24px; border-bottom:1px solid #f1f3f5; display:flex; justify-content:space-between; align-items:flex-end; font-family:'Inter', sans-serif;">
                                    <div style="font-family:'Inter', sans-serif;">
                                        <span style="font-size:11px; color:#adb5bd; text-transform:uppercase; font-weight:800; letter-spacing:0.5px; font-family:'Inter', sans-serif;">{"Kunde"}</span>
                                        <div style="font-size:28px; font-weight:800; color:#1a1a1a; margin-top:4px; font-family:'Inter', sans-serif;">{&a.auftraggeber.name}</div>
                                    </div>
                                    <div style="text-align:center; font-family:'Inter', sans-serif;">
                                        <span style="font-size:11px; color:#adb5bd; text-transform:uppercase; font-weight:800; letter-spacing:0.5px; font-family:'Inter', sans-serif;">{"Erstellt am"}</span>
                                        <div style="font-size:28px; font-weight:800; color:#1a1a1a; margin-top:4px; font-family:'Inter', sans-serif;">
                                            { 
                                                if a.start_datum.contains('-') {
                                                    let parts: Vec<&str> = a.start_datum.split('-').collect();
                                                    if parts.len() == 3 { format!("{}.{}.{}", parts[2], parts[1], parts[0]) }
                                                    else { a.start_datum.clone() }
                                                } else {
                                                    a.start_datum.clone()
                                                }
                                            }
                                        </div>
                                    </div>
                                    <div style="text-align:right; font-family:'Inter', sans-serif;">
                                        <span style="font-size:11px; color:#adb5bd; text-transform:uppercase; font-weight:800; letter-spacing:0.5px; font-family:'Inter', sans-serif;">{"Auftrags-Nr."}</span>
                                        <div style="font-size:28px; font-weight:800; color:#1a1a1a; margin-top:4px; font-family:'Inter', sans-serif;">{&a.auftrags_nummer}</div>
                                    </div>
                                </div>
                            </div>
                            <div style={card}><h3>{"📦 Koffer-Aufbau"}</h3><div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:10px; background:#f8f9fa; padding:15px; border-radius:12px;"><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"HERSTELLER"}</label><div style={inp}>{&a.koffer.hersteller}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"SERIENNUMMER"}</label><div style={inp}>{&a.koffer.seriennummer}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"BAUJAHR"}</label><div style={inp}>{a.koffer.baujahr}</div></div></div></div>
                            <div style={card}><h3>{"🚗 Altes Fahrzeug"}</h3><div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:10px; background:#f8f9fa; padding:15px; border-radius:12px;"><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"KENNZEICHEN"}</label><div style={inp}>{&a.spender_fahrgestell.kennzeichen}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"VIN"}</label><div style={inp}>{&a.spender_fahrgestell.vin}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"KILOMETERSTAND"}</label><div style={inp}>{format!("{:.0} km", a.spender_fahrgestell.kilometerstand)}</div></div></div></div>
                            <div style={card}><h3>{"🚚 Neues Fahrzeug"}</h3><div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:10px; background:#f8f9fa; padding:15px; border-radius:12px;"><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"KENNZEICHEN"}</label><div style={inp}>{&a.empfaenger_fahrgestell.kennzeichen}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"VIN"}</label><div style={inp}>{&a.empfaenger_fahrgestell.vin}</div></div><div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"KILOMETERSTAND"}</label><div style={inp}>{format!("{:.0} km", a.empfaenger_fahrgestell.kilometerstand)}</div></div></div></div>
                            <div style={card}>
                                <h3>{"🛠 Teileliste"}</h3>
                                <table style="width:100%; border-collapse:collapse; margin-bottom:20px;">
                                    <thead><tr style="text-align:left; border-bottom:2px solid #f1f3f5;"><th style="padding:10px;">{"Teil"}</th><th style="padding:10px;">{"Art.-Nr."}</th><th style="padding:10px;">{"Händler"}</th><th style="padding:10px; text-align:center;">{"Produktlink"}</th><th style="padding:10px; text-align:center;">{"Aktion"}</th></tr></thead>
                                    <tbody>
                                        { for a.teileliste.iter().enumerate().map(|(idx, t)| {
                                            let nr = nr_str.clone(); let m = management.clone(); let s = save.clone();
                                            html! {
                                                <tr style="border-bottom:1px solid #f1f3f5;">
                                                    <td style="padding:10px;">{&t.name}</td><td style="padding:10px;">{&t.artikel_nummer}</td><td style="padding:10px;">{&t.haendler}</td>
                                                    <td style="padding:10px; text-align:center;">{ if let Some(link) = &t.produkt_link { html! { <a href={link.clone()} target="_blank" style="color:#006666; text-decoration:none; font-size:18px;">{"🔗"}</a> } } else { html! { <span style="color:#dee2e6;">{"-"}</span> } }}</td>
                                                    <td style="padding:10px; text-align:center;"><button style="background:none; border:none; cursor:pointer; font-size:18px;" onclick={move |_| { let mut mm=(*m).clone(); if let Some(x)=mm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.teileliste.remove(idx); s.emit(mm); } }}>{"🗑️"}</button></td>
                                                </tr>
                                            }
                                        })}
                                    </tbody>
                                </table>
                                <div style="display:grid; grid-template-columns: 1fr 1fr 1fr 1fr auto; gap:10px; background:#f8f9fa; padding:15px; border-radius:12px;">
                                    <div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"BEZEICHNUNG"}</label><input id="new-part-name" style={inp} placeholder="Silikonfuge"/></div>
                                    <div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"ARTIKEL-NR."}</label><input id="new-part-art" style={inp} placeholder="44-998-12"/></div>
                                    <div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"HÄNDLER"}</label><input id="new-part-hnd" style={inp} placeholder="Fahrtec"/></div>
                                    <div style="display:flex; flex-direction:column; gap:4px;"><label style="font-size:10px; font-weight:800; color:#adb5bd; margin-left:4px;">{"PRODUKT-LINK"}</label><input id="new-part-link" style={inp} placeholder="https://..."/></div>
                                    <button style="padding:10px 20px; background:#006666; color:#fff; border:none; border-radius:10px; cursor:pointer; align-self:flex-end; margin-bottom:10px;" onclick={
                                        let nr=nr_str.clone(); let m=management.clone(); let s=save.clone();
                                        move |_| {
                                            let name = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-name").unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let art = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-art").unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let hnd = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-hnd").unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let link_val = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-link").unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let produkt_link = if link_val.is_empty() { None } else { Some(link_val) };
                                            if !name.is_empty() { let mut mm=(*m).clone(); if let Some(x)=mm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.teileliste.push(Bestellteil { name, artikel_nummer: art, haendler: hnd, produkt_link }); s.emit(mm); } }
                                        }
                                    }>{"Hinzufügen"}</button>
                                </div>
                            </div>
                            <div style={card}>
                                <h3>{"📸 Bilder & Dokumente"}</h3>
                                <div style="border: 2px dashed #dee2e6; border-radius: 12px; padding: 30px; text-align: center; color: #adb5bd; margin-bottom: 24px; cursor: pointer;"
                                    ondragover={Callback::from(|e: DragEvent| e.prevent_default())}
                                    ondrop={
                                        let nr=nr_str.clone(); let l=load.clone();
                                        Callback::from(move |e: DragEvent| {
                                            e.prevent_default();
                                            if let Some(data)=e.data_transfer() { if let Some(files)=data.files() {
                                                for i in 0..files.length() { if let Some(file)=files.get(i) {
                                                    let nr=nr.clone(); let l=l.clone();
                                                    spawn_local(async move {
                                                        let form=web_sys::FormData::new().unwrap(); let _=form.append_with_blob("file", &file);
                                                        let req=Request::post(&format!("http://127.0.0.1:3000/api/upload/{}", nr)).body(form);
                                                        if let Ok(r)=req { let _=r.send().await; l.emit(()); }
                                                    });
                                                } }
                                            } }
                                        })
                                    }
                                >{"Bilder hierher ziehen zum Hochladen"}</div>
                                <table style="width:100%; border-collapse:collapse;">
                                    <thead><tr style="text-align:left; border-bottom:2px solid #f1f3f5; color:#adb5bd; font-size:11px;"><th style="padding:10px;">{"DATEINAME"}</th><th style="padding:10px; text-align:center;">{"AKTIONEN"}</th></tr></thead>
                                    <tbody>
                                        { for a.bilder.iter().map(|f| {
                                            let img_url = format!("http://127.0.0.1:3000/api/images/{}", f);
                                            let sel1 = selected_image.clone(); let f_name1 = f.clone(); let nr1 = nr_str.clone(); let l1 = load.clone();
                                            html! { 
                                                <tr style="border-bottom:1px solid #f1f3f5;">
                                                    <td style="padding:12px; font-weight:600; color:#006666; cursor:pointer;" onclick={move |_| sel1.set(Some(f_name1.clone()))}>{f}</td>
                                                    <td style="padding:12px; text-align:center;"><div style="display:flex; gap:12px; justify-content:center; align-items:center;"><a href={img_url} target="_blank" style="text-decoration:none; font-size:18px;" title="In neuem Tab öffnen">{"💾"}</a><button style="background:none; border:none; cursor:pointer; font-size:18px;" onclick={let f_name=f.clone(); let nr=nr1.clone(); let l=l1.clone(); move |_| { if web_sys::window().unwrap().confirm_with_message(&format!("{} löschen?", f_name)).unwrap_or(false) { let f_name=f_name.clone(); let nr=nr.clone(); let l=l.clone(); spawn_local(async move { let _=Request::post(&format!("http://127.0.0.1:3000/api/delete-image/{}/{}", nr, f_name)).send().await; l.emit(()); }); } }}>{"🗑️"}</button></div></td>
                                                </tr>
                                            }
                                        }) }
                                    </tbody>
                                </table>
                            </div>
                            <div style={card}>
                                <label style="font-size:11px; font-weight:800; color:#adb5bd; text-transform:uppercase;">{"Umsatzerwartung (€)"}</label>
                                <input style={format!("{}; margin-top:8px; font-size:24px; font-weight:800; color:#006666;", inp)} type="number" value={a.umsatz.to_string()} oninput={let nr=nr_str.clone(); let m=management.clone(); let s=save.clone(); move |e: InputEvent| { let mut mm=(*m).clone(); if let Some(x)=mm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.umsatz=e.target_unchecked_into::<web_sys::HtmlSelectElement>().value().parse().unwrap_or(0.0); s.emit(mm); }}} />
                            </div>
                        </div>
                    }
                },
                None => html! { "Nicht gefunden" }
            }
        },
        Tab::NeuerAuftrag => {
            let mut kunden_liste: Vec<String> = vec!["Deutsches Rotes Kreuz".to_string(), "Malteser".to_string(), "Feuerwehr Leipzig".to_string()];
            kunden_liste.extend(management.auftraege.iter().map(|a| a.auftraggeber.name.clone()));
            kunden_liste.sort(); kunden_liste.dedup();
            let exists = management.auftraege.iter().any(|a| a.auftrags_nummer == *f_nr);
            let is_valid = !f_nr.is_empty() && !exists && !f_kn.is_empty() && !f_ks.is_empty() && !f_kh.is_empty() && !f_sv.is_empty() && !f_skz.is_empty() && !f_ev.is_empty() && !f_ekz.is_empty() && f_skm.is_some() && f_ekm.is_some() && f_kj.is_some();
            html! {
                <div style="width:100%;"><h1>{"📝 Neuer Auftrag"}</h1><div style={card}>
                    <div style="display:grid; grid-template-columns:1fr 1fr 1fr; gap:16px;">
                        <div style="display:flex; flex-direction:column; gap:4px;"><input style={format!("{} {}", inp, if exists { "border-color: #fa5252; background: #fff5f5;" } else { "" })} placeholder="Auftrags-Nummer" value={(*f_nr).clone()} oninput={let s=f_nr.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/>{ if exists { html! { <span style="font-size:10px; color:#fa5252; font-weight:700; margin-left:4px;">{"NUMMER BEREITS VERGEBEN"}</span> } } else { html! {} } }</div>
                        <div><input style={inp} list="kunden-liste" placeholder="Kunde auswählen oder neu eingeben" value={(*f_kn).clone()} oninput={let s=f_kn.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><datalist id="kunden-liste">{ for kunden_liste.iter().map(|k| html! { <option value={k.clone()} /> }) }</datalist></div>
                        <input style={inp} placeholder="Datum (TT.MM.JJJJ)" value={(*f_date).clone()} oninput={let s=f_date.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/>
                    </div>
                    <h3 style="margin-top:24px;">{"Koffer-Details"}</h3>
                    <div style="display:grid; grid-template-columns:1fr 1fr 1fr; gap:10px; margin-top:10px;">
                        <input style={inp} placeholder="Seriennummer Koffer" value={(*f_ks).clone()} oninput={let s=f_ks.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/>
                        <input style={inp} placeholder="Hersteller" value={(*f_kh).clone()} oninput={let s=f_kh.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}/>
                        <input style={inp} type="number" placeholder="Baujahr" value={f_kj.map(|v| v.to_string()).unwrap_or_default()} oninput={let s=f_kj.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok())}/>
                    </div>
                    <div style="display:grid; grid-template-columns:1fr 1fr; gap:20px; margin-top:10px;">
                        <div><div style="font-size:11px; font-weight:800; color:#fa5252;">{"ALTES FAHRZEUG"}</div><input style={inp} placeholder="VIN" value={(*f_sv).clone()} oninput={let s=f_sv.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}/><input style={inp} placeholder="Kennzeichen" value={(*f_skz).clone()} oninput={let s=f_skz.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}/><input style={inp} type="number" placeholder="Kilometerstand" value={f_skm.map(|v| v.to_string()).unwrap_or_default()} oninput={let s=f_skm.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok())}/></div>
                        <div><div style="font-size:11px; font-weight:800; color:#40c057;">{"NEUES FAHRZEUG"}</div><input style={inp} placeholder="VIN" value={(*f_ev).clone()} oninput={let s=f_ev.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}/><input style={inp} placeholder="Kennzeichen" value={(*f_ekz).clone()} oninput={let s=f_ekz.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}/><input style={inp} type="number" placeholder="Kilometerstand" value={f_ekm.map(|v| v.to_string()).unwrap_or_default()} oninput={let s=f_ekm.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().ok())}/></div>
                    </div>
                    <button style={format!("width:100%; padding:16px; border:none; border-radius:12px; font-weight:700; transition: all 0.2s; {}", if is_valid { "background:#006666; color:#fff; cursor:pointer;" } else { "background:#e9ecef; color:#adb5bd; cursor:not-allowed;" })} disabled={!is_valid} onclick={on_create}>{ if is_valid { "Auftrag anlegen" } else { "Bitte alle Felder ausfüllen" } }</button>
                </div></div>
            }
        },
        Tab::Archiv => {
            let archiv = management.auftraege.iter().filter(|a| a.status == AuftragsStatus::Abgeschlossen || a.status == AuftragsStatus::Storniert).collect::<Vec<_>>();
            html! {
                <div><h1>{"📦 Archiv"}</h1><p style="color:#666; margin-bottom:32px;">{"Abgeschlossene und stornierte Aufträge zur Dokumentation."}</p>
                    { if archiv.is_empty() { html! { <div style={card}>{"Keine archivierten Aufträge vorhanden."}</div> } } else {
                        html! { <table style="width:100%; background:#fff; border-radius:16px; border-collapse:collapse; overflow:hidden; box-shadow:0 4px 20px rgba(0,0,0,0.05);">
                                <thead><tr style="background:#f8f9fa; text-align:left; border-bottom:2px solid #f1f3f5;"><th style="padding:16px;">{"Nr."}</th><th style="padding:16px;">{"Kunde"}</th><th style="padding:16px;">{"Koffer"}</th><th style="padding:16px;">{"Umsatz"}</th><th style="padding:16px;">{"Status"}</th></tr></thead>
                                <tbody> { for archiv.iter().map(|a| {
                                    let nr=a.auftrags_nummer.clone(); let t=tab.clone();
                                    let (bg, color) = match a.status { AuftragsStatus::Abgeschlossen => ("#ebfbee", "#40c057"), AuftragsStatus::Storniert => ("#fff5f5", "#fa5252"), _ => ("#f1f3f5", "#495057") };
                                    html! { <tr style="border-bottom:1px solid #f1f3f5; cursor:pointer;" onclick={move |_| t.set(Tab::Details(nr.clone()))}><td style="padding:16px; font-weight:700;">{&a.auftrags_nummer}</td><td style="padding:16px;">{&a.auftraggeber.name}</td><td style="padding:16px;">{&a.koffer.seriennummer}</td><td style="padding:16px;">{format!("{:.0} €", a.umsatz)}</td><td style="padding:16px;"><span style={format!("background:{bg}; color:{color}; padding:4px 10px; border-radius:10px; font-size:11px; font-weight:700; text-transform:uppercase;")}>{format!("{:?}", a.status)}</span></td></tr> }
                                })} </tbody>
                            </table> }
                    }}
                </div>
            }
        }
    };

    html! {
        <div style="background:#f8f9fa; min-height:100vh; font-family:'Inter', sans-serif; display:flex;">
            <aside style="width:260px; background:#fff; border-right:1px solid #e9ecef; padding:32px 16px; position:fixed; height:100vh;">
                <h2 style="color:#006666; margin-bottom:40px; cursor:pointer;" onclick={let t=tab.clone(); move |_| t.set(Tab::Dashboard)}>{"Kofferwechsel-Software"}</h2>
                <nav>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Dashboard) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Dashboard)}>{"📊 Dashboard"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::NeuerAuftrag) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::NeuerAuftrag)}>{"📝 Neuer Auftrag"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Cockpit) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Cockpit)}>{"🚗 Auftragsverwaltung"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Archiv) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Archiv)}>{"📦 Archiv"}</button>
                </nav>
                <div style="position:absolute; bottom:32px; left:16px; font-size:12px; color:#adb5bd;">{ (*info).clone() }</div>
            </aside>
            <main style="flex:1; padding:40px; margin-left:260px;">{ content }</main>
            {
                if let Some(img) = &*selected_image {
                    let img_url = format!("http://127.0.0.1:3000/api/images/{}", img);
                    let sel = selected_image.clone();
                    html! {
                        <div style="position:fixed; top:0; left:0; width:100vw; height:100vh; background:rgba(0,0,0,0.9); display:flex; flex-direction:column; align-items:center; justify-content:center; z-index:10000; cursor:zoom-out; padding:20px; box-sizing:border-box;" onclick={move |_| sel.set(None)}>
                            <div style="position:relative; max-width:90%; display:flex; flex-direction:column; align-items:center;">
                                <img src={img_url.clone()} style="max-width:100%; max-height:70vh; border-radius:12px; box-shadow:0 10px 40px rgba(0,0,0,0.5); cursor:default; object-fit:contain;" onclick={|e: MouseEvent| e.stop_propagation()} />
                                <div style="margin-top:30px; display:flex; gap:16px; justify-content:center; width:100%;">
                                    <a href={img_url} download={img.clone()} style="background:#006666; color:#fff; padding:14px 28px; border-radius:12px; text-decoration:none; font-weight:700; display:flex; align-items:center; gap:10px; box-shadow:0 4px 12px rgba(0,0,0,0.2);" onclick={|e: MouseEvent| e.stop_propagation()}>{"💾 Bild herunterladen"}</a>
                                    <button style="background:#fff; color:#1a1a1a; padding:14px 28px; border-radius:12px; border:none; font-weight:700; cursor:pointer; box-shadow:0 4px 12px rgba(0,0,0,0.2);" onclick={let sel = selected_image.clone(); move |_| sel.set(None)}>{"Schließen"}</button>
                                </div>
                            </div>
                        </div>
                    }
                } else { html! {} }
            }
        </div>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
