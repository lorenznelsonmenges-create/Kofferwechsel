use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, File};
use std::collections::HashMap;
use rust_frontend::kofferwechsel::{KofferManagement, KofferwechselAuftrag, AuftragsStatus, Auftraggeber, Koffer, Fahrgestell, Bestellteil, BestellStatus};

#[derive(Clone, PartialEq)]
enum Tab { Dashboard, Cockpit, NeuerAuftrag, Details(String), Archiv }

fn s_badge(s: &AuftragsStatus) -> Html {
    let (bg, color, label) = match s {
        AuftragsStatus::Angenommen => ("#e7f5ff", "#228be6", "Angenommen"),
        AuftragsStatus::InArbeit => ("#fff4e6", "#fd7e14", "In Arbeit"),
        AuftragsStatus::Bereitstellung => ("#f3f0ff", "#7950f2", "Bereitstellung"),
        AuftragsStatus::Abgeschlossen => ("#ebfbee", "#40c057", "Abgeschlossen"),
        _ => ("#f8f9fa", "#868e96", "Status"),
    };
    html! { <span style={format!("background:{bg}; color:{color}; padding:4px 12px; border-radius:12px; font-size:12px; font-weight:700; text-transform:uppercase;")}>{label}</span> }
}

#[function_component(App)]
fn app() -> Html {
    let tab = use_state(|| Tab::Dashboard);
    let management = use_state(KofferManagement::new);
    let search = use_state(|| String::new());
    let info = use_state(|| String::new());

    let f_nr = use_state(|| String::new()); let f_kn = use_state(|| String::new());
    let f_ks = use_state(|| String::new()); let f_kh = use_state(|| String::new()); let f_kj = use_state(|| 2024u32);
    let f_sv = use_state(|| String::new()); let f_skz = use_state(|| String::new()); let f_skm = use_state(|| 0u32);
    let f_ev = use_state(|| String::new()); let f_ekz = use_state(|| String::new()); let f_ekm = use_state(|| 0u32);
    let f_ums = use_state(|| 0.0f64);

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
        Callback::from(move |_| {
            let mut nm = (*m).clone();
            let mut cl = HashMap::new();
            for i in ["Auspuff", "Retarder", "Schmutzfänger", "Beklebung", "VDE 0100", "O2-Probe"] { cl.insert(i.to_string(), false); }
            nm.auftraege.push(KofferwechselAuftrag {
                auftrags_nummer: (*fnr).clone(), status: AuftragsStatus::Angenommen, auftraggeber: Auftraggeber { name: (*fkn).clone(), kontakt: "".to_string() },
                koffer: Koffer { seriennummer: (*fks).clone(), hersteller: (*fkh).clone(), baujahr: *fkj },
                spender_fahrgestell: Fahrgestell { vin: (*fsv).clone(), kennzeichen: (*fskz).clone(), modell: "RTW".to_string(), kilometerstand: *fskm },
                empfaenger_fahrgestell: Fahrgestell { vin: (*fev).clone(), kennzeichen: (*fekz).clone(), modell: "RTW".to_string(), kilometerstand: *fekm },
                start_datum: "2024-03-24".to_string(), geplante_hochzeit: "".to_string(), abschluss_datum: None, umsatz: *fums, arbeitsstunden: 0.0, bilder: vec![], teileliste: vec![], checkliste: cl,
            });
            s.emit(nm); t.set(Tab::Cockpit);
        })
    };

    let card = "background:#fff; border-radius:16px; padding:24px; box-shadow:0 4px 20px rgba(0,0,0,0.05); margin-bottom:24px; box-sizing:border-box;";
    let inp = "padding:10px; border:1px solid #dee2e6; border-radius:10px; width:100%; margin-bottom:10px; font-size:14px; box-sizing:border-box;";

    let content = match &*tab {
        Tab::Dashboard => {
            let ab = management.auftraege.iter().filter(|a| a.status == AuftragsStatus::Abgeschlossen).collect::<Vec<_>>();
            let ak = management.auftraege.iter().filter(|a| a.status != AuftragsStatus::Abgeschlossen && a.status != AuftragsStatus::Archiviert).collect::<Vec<_>>();
            let gu: f64 = ab.iter().map(|a| a.umsatz).sum();
            let pu: f64 = ak.iter().map(|a| a.umsatz).sum();
            html! {
                <div>
                    <h1>{"Dashboard"}</h1>
                    <div style="display:grid; grid-template-columns: 1fr 1fr 1fr; gap:24px;">
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"UMSATZ (FERTIG)"}</div><div style="font-size:42px; font-weight:800; color:#006666;">{format!("{:.0} €", gu.abs())}</div></div>
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"PIPELINE (AKTIV)"}</div><div style="font-size:42px; font-weight:800; color:#fd7e14;">{format!("{:.0} €", pu.abs())}</div></div>
                        <div style={card}><div style="font-size:12px; color:#adb5bd;">{"Ø PRO AUFTRAG"}</div><div style="font-size:42px; font-weight:800; color:#228be6;">{format!("{:.0} €", (if ab.is_empty() {0.0} else {gu/ab.len() as f64}).abs())}</div></div>
                    </div>
                </div>
            }
        },
        Tab::Cockpit => {
            let q = (*search).to_lowercase();
            let gef = management.auftraege.iter().filter(|a| a.status != AuftragsStatus::Archiviert).filter(|a| a.auftrags_nummer.to_lowercase().contains(&q) || a.auftraggeber.name.to_lowercase().contains(&q)).collect::<Vec<_>>();
            html! {
                <div>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:32px;"><h1>{"🚗 Auftragsverwaltung"}</h1><input style="flex:1; max-width:400px; padding:12px 20px; border-radius:14px; border:1px solid #e9ecef;" placeholder="Suche..." oninput={let s=search.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/></div>
                    { for gef.iter().map(|a| {
                        let nr = a.auftrags_nummer.clone(); let t = tab.clone(); let ms = management.clone(); let ss = save.clone();
                        html! {
                            <div style={card}>
                                <div style="display:flex; justify-content:space-between;">
                                    <div><div style="font-size:12px; color:#666;">{&a.auftrags_nummer}</div><h2>{&a.auftraggeber.name}</h2></div>
                                    <div style="text-align:right;">
                                        <div style="font-size:20px; font-weight:800; color:#006666; margin-bottom:8px;">{format!("{:.0} €", a.umsatz)}</div>
                                        <select 
                                            style="padding:6px 12px; border-radius:12px; border:1px solid #dee2e6; font-size:12px; font-weight:700; text-transform:uppercase; cursor:pointer; background:#f8f9fa;"
                                            onchange={
                                                let nr = a.auftrags_nummer.clone(); let m = management.clone(); let s = save.clone();
                                                move |e: Event| {
                                                    let val = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                                    let status = match val.as_str() {
                                                        "Angenommen" => AuftragsStatus::Angenommen,
                                                        "InArbeit" => AuftragsStatus::InArbeit,
                                                        "Bereitstellung" => AuftragsStatus::Bereitstellung,
                                                        "Abgeschlossen" => AuftragsStatus::Abgeschlossen,
                                                        _ => AuftragsStatus::Angenommen,
                                                    };
                                                    let mut nm = (*m).clone();
                                                    if let Some(x) = nm.auftraege.iter_mut().find(|x| x.auftrags_nummer == nr) {
                                                        x.status = status;
                                                        s.emit(nm);
                                                    }
                                                }
                                            }
                                        >
                                            <option value="Angenommen" selected={a.status == AuftragsStatus::Angenommen}>{"Angenommen"}</option>
                                            <option value="InArbeit" selected={a.status == AuftragsStatus::InArbeit}>{"In Arbeit"}</option>
                                            <option value="Bereitstellung" selected={a.status == AuftragsStatus::Bereitstellung}>{"Bereitstellung"}</option>
                                            <option value="Abgeschlossen" selected={a.status == AuftragsStatus::Abgeschlossen}>{"Abgeschlossen"}</option>
                                        </select>
                                    </div>
                                </div>
                                <div style="display:grid; grid-template-columns:repeat(3, 1fr); gap:20px; font-size:14px; border-top:1px solid #f1f3f5; margin-top:16px; padding-top:16px;">
                                    <div><div style="font-size:11px; color:#adb5bd;">{"KOFFER-AUFBAU"}</div><div>{format!("{} (Seriennummer: {})", a.koffer.hersteller, a.koffer.seriennummer)}</div></div>
                                    <div><div style="font-size:11px; color:#adb5bd;">{"SPENDER-FAHRZEUG"}</div><div>{format!("{} ({} Kilometerstand)", a.spender_fahrgestell.kennzeichen, a.spender_fahrgestell.kilometerstand)}</div></div>
                                    <div><div style="font-size:11px; color:#adb5bd;">{"NEU-FAHRZEUG"}</div><div>{format!("{} ({} Kilometerstand)", a.empfaenger_fahrgestell.kennzeichen, a.empfaenger_fahrgestell.kilometerstand)}</div></div>
                                </div>
                                <div style="margin-top:20px; display:flex; gap:8px;">
                                    <button style="padding:8px 16px; background:#006666; color:#fff; border:none; border-radius:8px; cursor:pointer;" onclick={let t=t.clone(); let n=nr.clone(); move |_| t.set(Tab::Details(n.clone()))}>{"Spezifizieren"}</button>
                                </div>
                            </div>
                        }
                    })}
                </div>
            }
        },
        Tab::Details(nr) => {
            let a = management.auftraege.iter().find(|a| &a.auftrags_nummer == nr);
            match a {
                Some(a) => html! {
                    <div>
                        <button style="color:#006666; font-weight:700; cursor:pointer; background:none; border:none; margin-bottom:20px;" onclick={let t=tab.clone(); move |_| t.set(Tab::Cockpit)}>{"← Zurück"}</button>
                        <div style="display:grid; grid-template-columns: 2fr 1fr; gap:32px;">
                            <div>
                                <div style={card}>
                                    <h3>{"📸 Bilder"}</h3>
                                    <div 
                                        style="border: 2px dashed #dee2e6; border-radius: 12px; padding: 40px; text-align: center; color: #adb5bd; margin-bottom: 24px; cursor: pointer;"
                                        ondragover={Callback::from(|e: DragEvent| e.prevent_default())}
                                        ondrop={
                                            let nr = a.auftrags_nummer.clone(); let l = load.clone();
                                            Callback::from(move |e: DragEvent| {
                                                e.prevent_default();
                                                if let Some(data) = e.data_transfer() {
                                                    if let Some(files) = data.files() {
                                                        for i in 0..files.length() {
                                                            if let Some(file) = files.get(i) {
                                                                let nr = nr.clone(); let l = l.clone();
                                                                spawn_local(async move {
                                                                    let form = web_sys::FormData::new().unwrap();
                                                                    let _ = form.append_with_blob("file", &file);
                                                                    let req = Request::post(&format!("http://127.0.0.1:3000/api/upload/{}", nr)).body(form);
                                                                    if let Ok(r) = req { let _ = r.send().await; l.emit(()); }
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    >
                                        {"Bilder hierher ziehen zum Hochladen"}
                                    </div>
                                    <div style="display:grid; grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); gap:15px;">
                                        { for a.bilder.iter().map(|f| html! { <img src={format!("http://127.0.0.1:3000/api/images/{f}")} style="width:100%; border-radius:12px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);" /> }) }
                                    </div>
                                </div>
                            </div>
                            <div style={card}><label style="font-size:11px; font-weight:800; color:#adb5bd;">{"UMSATZ (€)"}</label><input style={inp} type="number" value={a.umsatz.to_string()} oninput={let nr=a.auftrags_nummer.clone(); let m=management.clone(); let s=save.clone(); move |e: InputEvent| { let mut mm=(*m).clone(); if let Some(x)=mm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) { x.umsatz=e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0.0); s.emit(mm); }}} /></div>
                        </div>
                        <div style={card}>
                            <h3>{"🛠 Teileliste"}</h3>
                            <table style="width:100%; border-collapse:collapse; margin-bottom:20px;">
                                <thead><tr style="text-align:left; border-bottom:2px solid #f1f3f5;"><th style="padding:10px;">{"Teil"}</th><th style="padding:10px;">{"Art.-Nr."}</th><th style="padding:10px;">{"Händler"}</th><th style="padding:10px;">{"Status"}</th></tr></thead>
                                <tbody>
                                    { for a.teileliste.iter().map(|t| html! {
                                        <tr style="border-bottom:1px solid #f1f3f5;"><td style="padding:10px;">{&t.name}</td><td style="padding:10px;">{&t.artikel_nummer}</td><td style="padding:10px;">{&t.haendler}</td><td style="padding:10px;">{format!("{:?}", t.status)}</td></tr>
                                    })}
                                </tbody>
                            </table>
                            <div style="display:grid; grid-template-columns: 2fr 1fr 1fr auto; gap:10px; background:#f8f9fa; padding:15px; border-radius:12px;">
                                <input id="new-part-name" style={inp} placeholder="Neues Teil..."/>
                                <input id="new-part-art" style={inp} placeholder="Art-Nr..."/>
                                <input id="new-part-hnd" style={inp} placeholder="Händler..."/>
                                <button style="padding:10px 20px; background:#006666; color:#fff; border:none; border-radius:10px; cursor:pointer;" onclick={
                                    let nr=a.auftrags_nummer.clone(); let m=management.clone(); let s=save.clone();
                                    move |_| {
                                        let name = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-name").unwrap().unchecked_into::<HtmlInputElement>().value();
                                        let art = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-art").unwrap().unchecked_into::<HtmlInputElement>().value();
                                        let hnd = web_sys::window().unwrap().document().unwrap().get_element_by_id("new-part-hnd").unwrap().unchecked_into::<HtmlInputElement>().value();
                                        if !name.is_empty() {
                                            let mut mm=(*m).clone();
                                            if let Some(x)=mm.auftraege.iter_mut().find(|x| x.auftrags_nummer==nr) {
                                                x.teileliste.push(Bestellteil { name, artikel_nummer: art, haendler: hnd, status: BestellStatus::Offen });
                                                s.emit(mm);
                                            }
                                        }
                                    }
                                }>{"Hinzufügen"}</button>
                            </div>
                        </div>
                    </div>
                },
                None => html! { "Nicht gefunden" }
            }
        },
        Tab::NeuerAuftrag => html! {
            <div style="width:100%;">
                <h1>{"Neuer Kofferwechsel-Auftrag"}</h1>
                <div style={card}>
                    <div style="display:grid; grid-template-columns:1fr 1fr; gap:16px;"><input style={inp} placeholder="Auftrags-Nummer" oninput={let s=f_nr.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} placeholder="Kunde" oninput={let s=f_kn.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/></div>
                    <h3 style="margin-top:24px;">{"Koffer-Details"}</h3>

                    <div style="display:grid; grid-template-columns:1fr 1fr 1fr; gap:10px; margin-top:10px;"><input style={inp} placeholder="Seriennummer Koffer" oninput={let s=f_ks.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} placeholder="Hersteller" oninput={let s=f_kh.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} type="number" placeholder="Baujahr" oninput={let s=f_kj.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(2024))}/></div>
                    <div style="display:grid; grid-template-columns:1fr 1fr; gap:20px; margin-top:10px;">
                        <div><div style="font-size:11px; font-weight:800; color:#fa5252;">{"SPENDER-FAHRZEUG (ALT)"}</div><input style={inp} placeholder="VIN" oninput={let s=f_sv.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} placeholder="Kennzeichen" oninput={let s=f_skz.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} type="number" placeholder="Kilometerstand" oninput={let s=f_skm.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0))}/></div>
                        <div><div style="font-size:11px; font-weight:800; color:#40c057;">{"NEU-FAHRZEUG (EMPFÄNGER)"}</div><input style={inp} placeholder="VIN" oninput={let s=f_ev.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} placeholder="Kennzeichen" oninput={let s=f_ekz.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value())}/><input style={inp} type="number" placeholder="Kilometerstand" oninput={let s=f_ekm.clone(); move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0))}/></div>
                    </div>
                    <button style="width:100%; padding:16px; background:#006666; color:#fff; border:none; border-radius:12px; font-weight:700; cursor:pointer;" onclick={on_create}>{"Auftrag anlegen"}</button>
                </div>
            </div>
        },
        Tab::Archiv => html! { "Archiv folgt" }
    };

    html! {
        <div style="background:#f8f9fa; min-height:100vh; font-family:'Inter', sans-serif; display:flex;">
            <aside style="width:260px; background:#fff; border-right:1px solid #e9ecef; padding:32px 16px; position:fixed; height:100vh;">
                <h2 style="color:#006666; margin-bottom:40px;">{"Kofferwechsel"}</h2>
                <nav>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Dashboard) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Dashboard)}>{"📊 Dashboard"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::NeuerAuftrag) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::NeuerAuftrag)}>{"📝 Neuer Auftrag"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Cockpit) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Cockpit)}>{"🚗 Auftragsverwaltung"}</button>
                    <button style={format!("width:100%; padding:12px 16px; text-align:left; border:none; border-radius:12px; cursor:pointer; font-weight:600; margin-bottom:8px; {}", if matches!(*tab, Tab::Archiv) { "background:#006666; color:#fff;" } else { "background:transparent;" })} onclick={let t=tab.clone(); move |_| t.set(Tab::Archiv)}>{"📦 Archiv"}</button>
                </nav>
                <div style="position:absolute; bottom:32px; left:16px; font-size:12px; color:#adb5bd;">{ (*info).clone() }</div>
            </aside>
            <main style="flex:1; padding:40px; margin-left:260px;">{ content }</main>
        </div>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
