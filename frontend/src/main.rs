use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use rust_frontend::carsharing::{
    Car, CarSharing, CarSharingService, CarStatus, Person, PersonStatus,
};

#[derive(Clone, PartialEq)]
enum Tab {
    FleetOverview,
    Persons,
    Cars,
    Reservations,
    Rentals,
    Simulation,
}

fn sidebar_button(current: &Tab, tab: Tab, icon: &str, label: &str, on_click: Callback<MouseEvent>) -> Html {
    let is_active = *current == tab;
    let style = if is_active {
        "display:flex; align-items:center; gap:12px; padding:12px 16px; background:#fff; color:#000; border-radius:12px; cursor:pointer; border:none; width:100%; text-align:left; font-weight:600; box-shadow: 0 2px 4px rgba(0,0,0,0.05);"
    } else {
        "display:flex; align-items:center; gap:12px; padding:12px 16px; background:transparent; color:#666; border-radius:12px; cursor:pointer; border:none; width:100%; text-align:left; font-weight:500;"
    };

    html! { 
        <button style={style} onclick={on_click}>
            <span style="font-size:20px;">{icon}</span>
            {label}
        </button> 
    }
}

#[function_component(App)]
fn app() -> Html {
    let tab = use_state(|| Tab::FleetOverview);
    let cs = use_state(CarSharing::new);

    // --- LADE-MECHANISMUS ---
    {
        let cs = cs.clone();
        use_effect(move || {
            spawn_local(async move {
                let fetched_cs: CarSharing = Request::get("/api/state")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                cs.set(fetched_cs);
            });
            || ()
        });
    }

    // --- NEUER SPEICHER-MECHANISMUS ---
    let save_state = {
        let info = use_state(|| String::new());
        Callback::from(move |model: CarSharing| {
            let info = info.clone();
            spawn_local(async move {
                let request = Request::post("/api/state")
                    .json(&model);
                
                match request {
                    Ok(req) => {
                        if let Err(_) = req.send().await {
                            info.set("Fehler: Konnte Zustand nicht ans Backend senden.".to_string());
                        }
                    }
                    Err(_) => {
                        info.set("Fehler: Interner Fehler beim Erstellen der Anfrage.".to_string());
                    }
                }
            });
        })
    };

    let info = use_state(|| String::new());

    // ---------- Form States ----------
    let p_id = use_state(|| "".to_string());
    let p_days = use_state(|| "".to_string());
    let c_id = use_state(|| "".to_string());
    let c_km = use_state(|| "".to_string());
    let c_age = use_state(|| "".to_string());
    let r_person = use_state(|| "".to_string());
    let r_car = use_state(|| "".to_string());
    let r_prio = use_state(|| "".to_string());
    let ret_person = use_state(|| "".to_string());
    let ret_car = use_state(|| "".to_string());
    let ret_km = use_state(|| "".to_string());
    let sim_days = use_state(|| "".to_string());

    let on_reset = {
        let cs = cs.clone();
        let info = info.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let new_model = CarSharing::new();
            save_state.emit(new_model.clone()); 
            cs.set(new_model);
            info.set("State an Backend gesendet und zurückgesetzt.".to_string());
        })
    };

    // ========== Tab Switch Callbacks ==========
    let set_tab_overview = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::FleetOverview)) };
    let set_tab_persons = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::Persons)) };
    let set_tab_cars = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::Cars)) };
    let set_tab_res = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::Reservations)) };
    let set_tab_rentals = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::Rentals)) };
    let set_tab_sim = { let tab = tab.clone(); Callback::from(move |_: MouseEvent| tab.set(Tab::Simulation)) };

    // ========== Simulation Action (Unified) ==========
    let run_simulation = {
        let cs = cs.clone();
        let info = info.clone();
        let save_state = save_state.clone();
        Callback::from(move |days: u32| {
            let mut model = (*cs).clone();
            model.simulate_n_days(days);
            save_state.emit(model.clone());
            cs.set(model);
            info.set(format!("Simulation durchgeführt: {} Tage.", days));
        })
    };

    let on_simulate_custom = {
        let run_sim = run_simulation.clone();
        let sim_days = sim_days.clone();
        let info = info.clone();
        Callback::from(move |_: MouseEvent| {
            if let Ok(n) = (*sim_days).trim().parse::<u32>() {
                run_sim.emit(n);
            } else {
                info.set("Bitte gültige Anzahl an Tagen eingeben.".to_string());
            }
        })
    };

    // ========== Persons Actions ==========
    let on_add_person = {
        let cs = cs.clone();
        let info = info.clone();
        let p_id = p_id.clone();
        let p_days = p_days.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let id = (*p_id).trim().to_string();
            if id.is_empty() { info.set("Person-ID darf nicht leer sein.".to_string()); return; }
            let days = match (*p_days).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("license_valid_days muss eine Zahl sein.".to_string()); return; }
            };
            let ok = model.register_person(Person { identifier: id.clone(), license_valid_days: days, status: PersonStatus::Active });
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Person '{}' angelegt.", id));
            } else {
                info.set("Person existiert schon oder konnte nicht angelegt werden.".to_string());
            }
        })
    };

    let on_remove_person = {
        let cs = cs.clone();
        let info = info.clone();
        let p_id = p_id.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let id = (*p_id).trim().to_string();
            if id.is_empty() { info.set("Zum Entfernen bitte Person-ID eingeben.".to_string()); return; }
            let ok = model.unregister_person(&id);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Person '{}' entfernt.", id));
            } else {
                info.set("Person konnte nicht entfernt werden.".to_string());
            }
        })
    };

    let on_renew_license = {
        let cs = cs.clone();
        let info = info.clone();
        let p_id = p_id.clone();
        let p_days = p_days.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let id = (*p_id).trim().to_string();
            if id.is_empty() { info.set("Bitte Person-ID eingeben.".to_string()); return; }
            let days = match (*p_days).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("new_valid_days muss eine Zahl sein.".to_string()); return; }
            };
            let ok = model.renew_license(&id, days);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Führerschein für '{}' erneuert.", id));
            } else {
                info.set("Person nicht gefunden.".to_string());
            }
        })
    };

    // ========== Cars Actions ==========
    let on_add_car = {
        let cs = cs.clone();
        let info = info.clone();
        let c_id = c_id.clone();
        let c_km = c_km.clone();
        let c_age = c_age.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let id = (*c_id).trim().to_string();
            if id.is_empty() { info.set("Car-ID darf nicht leer sein.".to_string()); return; }
            let mileage = match (*c_km).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("mileage muss eine Zahl sein.".to_string()); return; }
            };
            let age_days = match (*c_age).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("age_days muss eine Zahl sein.".to_string()); return; }
            };
            let ok = model.register_car(Car { identifier: id.clone(), mileage, status: CarStatus::Available, age_days, rental_count: 0 });
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Auto '{}' angelegt.", id));
            } else {
                info.set("Auto konnte nicht angelegt werden.".to_string());
            }
        })
    };

    let on_remove_car = {
        let cs = cs.clone();
        let info = info.clone();
        let c_id = c_id.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let id = (*c_id).trim().to_string();
            if id.is_empty() { info.set("Zum Entfernen bitte Car-ID eingeben.".to_string()); return; }
            let ok = model.unregister_car(&id);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Auto '{}' entfernt.", id));
            } else {
                info.set("Auto konnte nicht entfernt werden.".to_string());
            }
        })
    };

    // ========== Reservation Actions ==========
    let on_reserve = {
        let cs = cs.clone();
        let info = info.clone();
        let r_person = r_person.clone();
        let r_car = r_car.clone();
        let r_prio = r_prio.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let person_id = (*r_person).trim().to_string();
            let car_id = (*r_car).trim().to_string();
            if person_id.is_empty() || car_id.is_empty() { info.set("Bitte Person-ID und Car-ID eingeben.".to_string()); return; }
            let prio = match (*r_prio).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("priority muss eine Zahl sein.".to_string()); return; }
            };
            let ok = model.reserve_car(&person_id, &car_id, prio);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Reservierung gesetzt: {} -> {}.", person_id, car_id));
            } else {
                info.set("Reservierung nicht möglich.".to_string());
            }
        })
    };

    let on_cancel_reservation = {
        let cs = cs.clone();
        let info = info.clone();
        let r_person = r_person.clone();
        let r_car = r_car.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let person_id = (*r_person).trim().to_string();
            let car_id = (*r_car).trim().to_string();
            if person_id.is_empty() || car_id.is_empty() { info.set("Bitte Person-ID und Car-ID eingeben.".to_string()); return; }
            let ok = model.cancel_reservation(&person_id, &car_id);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Reservierung storniert: {} -> {}.", person_id, car_id));
            } else {
                info.set("Reservierung nicht gefunden.".to_string());
            }
        })
    };


    // ========== Return Action ==========
    let on_return = {
        let cs = cs.clone();
        let info = info.clone();
        let ret_person = ret_person.clone();
        let ret_car = ret_car.clone();
        let ret_km = ret_km.clone();
        let save_state = save_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut model = (*cs).clone();
            let person_id = (*ret_person).trim().to_string();
            let car_id = (*ret_car).trim().to_string();
            if person_id.is_empty() || car_id.is_empty() { info.set("Bitte Person-ID und Car-ID eingeben.".to_string()); return; }
            let driven_km = match (*ret_km).trim().parse::<u32>() {
                Ok(v) => v,
                Err(_) => { info.set("driven_km muss eine Zahl sein.".to_string()); return; }
            };
            let ok = model.return_car(&person_id, &car_id, driven_km);
            if ok {
                save_state.emit(model.clone());
                cs.set(model);
                info.set(format!("Auto zurückgegeben: {} -> {}.", person_id, car_id));
            } else {
                info.set("Return fehlgeschlagen.".to_string());
            }
        })
    };

    // ========== Inputs: oninput callbacks ==========
    let on_p_id = { let p_id = p_id.clone(); Callback::from(move |e: InputEvent| { p_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_p_days = { let p_days = p_days.clone(); Callback::from(move |e: InputEvent| { p_days.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_c_id = { let c_id = c_id.clone(); Callback::from(move |e: InputEvent| { c_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_c_km = { let c_km = c_km.clone(); Callback::from(move |e: InputEvent| { c_km.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_c_age = { let c_age = c_age.clone(); Callback::from(move |e: InputEvent| { c_age.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_r_person = { let r_person = r_person.clone(); Callback::from(move |e: InputEvent| { r_person.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_r_car = { let r_car = r_car.clone(); Callback::from(move |e: InputEvent| { r_car.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_r_prio = { let r_prio = r_prio.clone(); Callback::from(move |e: InputEvent| { r_prio.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_ret_person = { let ret_person = ret_person.clone(); Callback::from(move |e: InputEvent| { ret_person.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_ret_car = { let ret_car = ret_car.clone(); Callback::from(move |e: InputEvent| { ret_car.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_ret_km = { let ret_km = ret_km.clone(); Callback::from(move |e: InputEvent| { ret_km.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };
    let on_sim_days = { let sim_days = sim_days.clone(); Callback::from(move |e: InputEvent| { sim_days.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()); }) };

    // ========== Render current tab ==========
    let current_tab = (*tab).clone();
    let model = (*cs).clone();
    
    // CSS Constants for the new Look
    let main_bg = "background-color: #f8f9fa; min-height: 100vh; display: flex; font-family: 'Inter', system-ui, sans-serif; color: #1a1a1a;";
    let sidebar_style = "width: 260px; background-color: #f1f3f5; padding: 24px 16px; display: flex; flex-direction: column; gap: 8px; border-right: 1px solid #e9ecef;";
    let content_area = "flex: 1; padding: 40px; overflow-y: auto;";
    
    let card_style = "background: #fff; border-radius: 20px; padding: 24px; box-shadow: 0 4px 6px rgba(0,0,0,0.02); border: 1px solid #f1f3f5;";
    let stats_grid = "display: grid; grid-template-columns: repeat(4, 1fr); gap: 24px; margin-bottom: 40px;";
    
    let btn_primary = "padding: 12px 20px; background: #006666; color: #fff; border: none; border-radius: 12px; font-weight: 600; cursor: pointer; display: flex; align-items: center; justify-content: space-between; width: 100%; transition: opacity 0.2s;";
    let btn_secondary = "padding: 12px 20px; background: #d0d7ff; color: #5c7cfa; border: none; border-radius: 12px; font-weight: 600; cursor: pointer; display: flex; align-items: center; justify-content: space-between; width: 100%; transition: opacity 0.2s;";
    let btn_outline = "padding: 12px 20px; background: #e9ecef; color: #495057; border: none; border-radius: 12px; font-weight: 600; cursor: pointer; display: flex; align-items: center; justify-content: space-between; width: 100%; transition: opacity 0.2s;";

    let content = match current_tab {
        Tab::FleetOverview => {
            let available = model.cars.iter().filter(|c| c.status == CarStatus::Available).count();
            let rented = model.rentals.len();
            let maintenance = model.cars.iter().filter(|c| matches!(c.status, CarStatus::Maintenance(_))).count();
            let tuv = model.cars.iter().filter(|c| matches!(c.status, CarStatus::Tuv(_))).count();

            html! {
                <div>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom: 8px;">
                        <h1 style="font-size: 32px; font-weight: 800; margin: 0;">{"Fleet Overview"}</h1>
                        <div style="background:#e6fcf5; color:#0ca678; padding:6px 12px; border-radius:20px; font-size:14px; font-weight:600; display:flex; align-items:center; gap:6px;">
                            <span style="width:8px; height:8px; background:#0ca678; border-radius:50%;"></span>
                            {"System Live"}
                        </div>
                    </div>
                    <p style="color: #666; margin-bottom: 32px;">{"Real-time monitoring and logistics simulation engine."}</p>

                    <div style={stats_grid}>
                        <div style={card_style}>
                            <div style="display:flex; justify-content:space-between; align-items:start; margin-bottom:20px;">
                                <div style="background:#e6fcf5; color:#0ca678; padding:4px 8px; border-radius:6px; font-size:12px; font-weight:700;">{"AVAILABLE"}</div>
                                <span style="color:#0ca678; font-size:24px;">{"✓"}</span>
                            </div>
                            <div style="font-size:48px; font-weight:800;">{available}</div>
                            <div style="color:#666; font-size:12px; font-weight:600; text-transform:uppercase; letter-spacing:0.5px;">{"Ready for service"}</div>
                        </div>
                        <div style={card_style}>
                            <div style="display:flex; justify-content:space-between; align-items:start; margin-bottom:20px;">
                                <div style="background:#edf2ff; color:#4c6ef5; padding:4px 8px; border-radius:6px; font-size:12px; font-weight:700;">{"RENTED"}</div>
                                <span style="color:#4c6ef5; font-size:24px;">{"⟳"}</span>
                            </div>
                            <div style="font-size:48px; font-weight:800;">{rented}</div>
                            <div style="color:#666; font-size:12px; font-weight:600; text-transform:uppercase; letter-spacing:0.5px;">{"Active rentals"}</div>
                        </div>
                        <div style={card_style}>
                            <div style="display:flex; justify-content:space-between; align-items:start; margin-bottom:20px;">
                                <div style="background:#fff5f5; color:#fa5252; padding:4px 8px; border-radius:6px; font-size:12px; font-weight:700;">{"MAINTENANCE"}</div>
                                <span style="color:#fa5252; font-size:24px;">{"🔧"}</span>
                            </div>
                            <div style="font-size:48px; font-weight:800;">{maintenance}</div>
                            <div style="color:#666; font-size:12px; font-weight:600; text-transform:uppercase; letter-spacing:0.5px;">{"In workshop"}</div>
                        </div>
                        <div style={card_style}>
                            <div style="display:flex; justify-content:space-between; align-items:start; margin-bottom:20px;">
                                <div style="background:#fff9db; color:#f08c00; padding:4px 8px; border-radius:6px; font-size:12px; font-weight:700;">{"URGENT"}</div>
                                <span style="color:#f08c00; font-size:24px;">{"!"}</span>
                            </div>
                            <div style="font-size:48px; font-weight:800;">{tuv}</div>
                            <div style="color:#666; font-size:12px; font-weight:600; text-transform:uppercase; letter-spacing:0.5px;">{"TUV expiring soon"}</div>
                        </div>
                    </div>

                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                        <div style={format!("{}; min-height:400px; display:flex; flex-direction:column;", card_style)}>
                            <h3 style="margin:0 0 8px 0;">{"Active Rentals"}</h3>
                            <p style="color:#666; font-size:14px; margin-bottom:24px;">{"Currently active vehicle deployments."}</p>
                            
                            <div style="flex:1; overflow-y:auto; padding-right:8px;">
                                { if model.rentals.is_empty() {
                                    html! { <div style="color:#adb5bd; font-size:14px; text-align:center; margin-top:40px;">{"No active rentals at the moment."}</div> }
                                } else {
                                    html! {
                                        <ul style="list-style:none; padding:0; margin:0; display:flex; flex-direction:column; gap:12px;">
                                            { for model.rentals.iter().map(|(p, c)| html! {
                                                <li style="display:flex; align-items:center; gap:12px; padding:12px; background:#f8f9fa; border-radius:12px; border:1px solid #f1f3f5;">
                                                    <div style="width:32px; height:32px; background:#e9ecef; border-radius:50%; display:flex; align-items:center; justify-content:center; font-size:14px;">{"🚗"}</div>
                                                    <div style="flex:1;">
                                                        <div style="font-weight:700; font-size:14px;">{c.to_string()}</div>
                                                        <div style="font-size:12px; color:#666;">{format!("Driver: {}", p)}</div>
                                                    </div>
                                                    <div style="background:#e7f5ff; color:#228be6; font-size:10px; font-weight:800; padding:4px 8px; border-radius:6px; text-transform:uppercase;">{"On Track"}</div>
                                                </li>
                                            })}
                                        </ul>
                                    }
                                }}
                            </div>
                        </div>

                        <div style={card_style}>
                            <h3 style="margin:0 0 8px 0;">{"Simulate Operations"}</h3>
                            <p style="color:#666; font-size:14px; margin-bottom:24px;">{"Fast-forward fleet lifecycle to test maintenance thresholds."}</p>
                            
                            <div style="display:flex; flex-direction:column; gap:12px;">
                                <button style={btn_primary} onclick={let r=run_simulation.clone(); move |_| r.emit(1)}>
                                    {"Simulate 1 Day"} <span style="font-size:18px;">{"▶▶"}</span>
                                </button>
                                <button style={btn_secondary} onclick={let r=run_simulation.clone(); move |_| r.emit(7)}>
                                    {"Simulate 7 Days"} <span style="font-size:18px;">{"≫"}</span>
                                </button>
                                <button style={btn_outline} onclick={let r=run_simulation.clone(); move |_| r.emit(30)}>
                                    {"Simulate 30 Days"} <span style="font-size:18px;">{"⟳"}</span>
                                </button>
                            </div>

                            <div style="margin-top:32px;">
                                <label style="display:block; font-size:11px; font-weight:800; color:#495057; text-transform:uppercase; margin-bottom:8px;">{"Custom Interval"}</label>
                                <div style="display:flex; gap:8px;">
                                    <input 
                                        style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px; font-size:14px;" 
                                        placeholder="Days" 
                                        value={(*sim_days).clone()} 
                                        oninput={on_sim_days}
                                    />
                                    <button 
                                        style="padding:0 16px; background:#006666; color:#fff; border:none; border-radius:12px; cursor:pointer;"
                                        onclick={on_simulate_custom}
                                    >{"▶"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        },
        Tab::Persons => html! {
            <div>
                <h1 style="margin-bottom:32px;">{"Persons Management"}</h1>
                <div style={format!("{}; margin-bottom:24px;", card_style)}>
                    <div style="display:flex; gap:16px; margin-bottom:16px;">
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Person-ID" value={(*p_id).clone()} oninput={on_p_id}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="License Valid Days" value={(*p_days).clone()} oninput={on_p_days}/>
                    </div>
                    <div style="display:flex; gap:12px;">
                        <button style="flex:1; padding:12px; background:#006666; color:#fff; border:none; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_add_person}>{"Add Person"}</button>
                        <button style="flex:1; padding:12px; background:#fff; border:1px solid #dee2e6; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_remove_person}>{"Remove Person"}</button>
                        <button style="flex:1; padding:12px; background:#fff; border:1px solid #dee2e6; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_renew_license}>{"Renew License"}</button>
                    </div>
                </div>
                <div style={card_style}>
                    <h3 style="margin-bottom:16px;">{format!("Registered Persons ({})", model.persons.len())}</h3>
                    <ul style="list-style:none; padding:0;">
                        { for model.persons.iter().map(|p| html!{ 
                            <li style="padding:12px; border-bottom:1px solid #f1f3f5; display:flex; justify-content:space-between;">
                                <span style="font-weight:600;">{p.identifier.clone()}</span>
                                <span style="color:#666;">{format!("Valid: {} days | Status: {:?}", p.license_valid_days, p.status)}</span>
                            </li> 
                        }) }
                    </ul>
                </div>
            </div>
        },
        Tab::Cars => html! {
            <div>
                <h1 style="margin-bottom:32px;">{"Fleet Management"}</h1>
                <div style={format!("{}; margin-bottom:24px;", card_style)}>
                    <div style="display:flex; gap:16px; margin-bottom:16px;">
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Car-ID" value={(*c_id).clone()} oninput={on_c_id}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Mileage (km)" value={(*c_km).clone()} oninput={on_c_km}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Age (days)" value={(*c_age).clone()} oninput={on_c_age}/>
                    </div>
                    <div style="display:flex; gap:12px;">
                        <button style="flex:1; padding:12px; background:#006666; color:#fff; border:none; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_add_car}>{"Add Vehicle"}</button>
                        <button style="flex:1; padding:12px; background:#fff; border:1px solid #dee2e6; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_remove_car}>{"Remove Vehicle"}</button>
                    </div>
                </div>
                <div style={card_style}>
                    <h3 style="margin-bottom:16px;">{format!("Total Assets ({})", model.cars.len())}</h3>
                    <ul style="list-style:none; padding:0;">
                        { for model.cars.iter().map(|c| html!{ 
                            <li style="padding:12px; border-bottom:1px solid #f1f3f5; display:flex; justify-content:space-between;">
                                <div>
                                    <span style="font-weight:600; margin-right:12px;">{c.identifier.clone()}</span>
                                    <span style="font-size:12px; background:#f1f3f5; padding:2px 8px; border-radius:4px;">{format!("{:?}", c.status)}</span>
                                </div>
                                <span style="color:#666;">{format!("{} km | {} days old", c.mileage, c.age_days)}</span>
                            </li> 
                        }) }
                    </ul>
                </div>
            </div>
        },
        Tab::Reservations => html! {
            <div>
                <h1 style="margin-bottom:32px;">{"Reservations"}</h1>
                <div style={format!("{}; margin-bottom:24px;", card_style)}>
                    <div style="display:flex; gap:16px; margin-bottom:16px;">
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Person-ID" value={(*r_person).clone()} oninput={on_r_person}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Car-ID" value={(*r_car).clone()} oninput={on_r_car}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Priority (1(niedrigste) -100(höchste))" value={(*r_prio).clone()} oninput={on_r_prio}/>
                    </div>
                    <div style="display:flex; gap:12px;">
                        <button style="flex:1; padding:12px; background:#006666; color:#fff; border:none; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_reserve}>{"Create Reservation"}</button>
                        <button style="flex:1; padding:12px; background:#fff; border:1px solid #dee2e6; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_cancel_reservation}>{"Cancel"}</button>
                    </div>
                </div>
                <div style={card_style}>
                    <h3 style="margin-bottom:16px;">{format!("Pending Queue ({})", model.reservations.len())}</h3>
                    <ul style="list-style:none; padding:0;">
                        { for model.reservations.iter().map(|r| html!{ 
                            <li style="padding:12px; border-bottom:1px solid #f1f3f5; display:flex; justify-content:space-between;">
                                <span><span style="font-weight:600;">{r.person_id.clone()}</span>{" → "}{r.car_id.clone()}</span>
                                <span style="color:#666; font-size:12px;">{format!("Priority: {}", r.priority)}</span>
                            </li> 
                        }) }
                    </ul>
                </div>
            </div>
        },
        Tab::Rentals => html! {
            <div>
                <h1 style="margin-bottom:32px;">{"Active Rentals"}</h1>
                <div style={format!("{}; margin-bottom:24px;", card_style)}>
                    <h3 style="margin-bottom:16px;">{"Return Vehicle"}</h3>
                    <div style="display:flex; gap:16px; margin-bottom:16px;">
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Person-ID" value={(*ret_person).clone()} oninput={on_ret_person}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Car-ID" value={(*ret_car).clone()} oninput={on_ret_car}/>
                        <input style="flex:1; padding:12px; border:1px solid #dee2e6; border-radius:12px;" placeholder="Driven KM" value={(*ret_km).clone()} oninput={on_ret_km}/>
                    </div>
                    <button style="width:100%; padding:12px; background:#006666; color:#fff; border:none; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_return}>{"Process Return"}</button>
                </div>
                <div style={card_style}>
                    <h3 style="margin-bottom:16px;">{format!("Currently Rented ({})", model.rentals.len())}</h3>
                    <ul style="list-style:none; padding:0;">
                        { for model.rentals.iter().map(|(p,c)| html!{ 
                            <li style="padding:12px; border-bottom:1px solid #f1f3f5; font-weight:600;">{format!("{} is driving {}", p, c)}</li> 
                        }) }
                    </ul>
                </div>
            </div>
        },
        Tab::Simulation => html! {
            <div>
                <h1 style="margin-bottom:32px;">{"Simulation Settings"}</h1>
                <div style={card_style}>
                    <p style="color:#666; margin-bottom:24px;">{"Manage global state and advanced simulation parameters."}</p>
                    <div style="display:flex; gap:12px;">
                        <button style="flex:1; padding:12px; background:#fa5252; color:#fff; border:none; border-radius:12px; font-weight:600; cursor:pointer;" onclick={on_reset}>{"Master Reset State"}</button>
                    </div>
                    <p style="margin-top:24px; font-size:14px; color:#adb5bd;">{format!("System Epoch: Day {}", model.current_day)}</p>
                </div>
            </div>
        },
    };

    html! {
        <div style={main_bg}>
            <aside style={sidebar_style}>
                <div style="margin-bottom: 40px; padding: 0 16px;">
                    <h2 style="font-size: 24px; font-weight: 800; margin: 0;">{"Carsharing"}</h2>
                    <p style="font-size: 11px; font-weight: 800; color: #adb5bd; text-transform: uppercase; letter-spacing: 1px; margin: 4px 0 0 0;">{"Internal Console"}</p>
                </div>
                
                { sidebar_button(&tab, Tab::FleetOverview, "📊", "Fleet Overview", set_tab_overview) }
                { sidebar_button(&tab, Tab::Persons, "👥", "Persons", set_tab_persons) }
                { sidebar_button(&tab, Tab::Cars, "🚗", "Cars", set_tab_cars) }
                { sidebar_button(&tab, Tab::Reservations, "📅", "Reservations", set_tab_res) }
                { sidebar_button(&tab, Tab::Rentals, "⇄", "Active Rentals", set_tab_rentals) }

            </aside>

            <main style={content_area}>
                {content}
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}