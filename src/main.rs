use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use js_sys::{Array, Reflect};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, thread_local_v2)]
    static PEDS_DATA: JsValue;

    #[wasm_bindgen(js_namespace = window, thread_local_v2)]
    static PEDS_TOC: JsValue;
}

#[derive(Properties, PartialEq)]
struct AppProps {}

fn safe_html(html_str: &str) -> Html {
    let wrapped = format!("<span>{}</span>", html_str);
    Html::from_html_unchecked(AttrValue::from(wrapped))
}

#[function_component(App)]
fn app() -> Html {
    let weight = use_state(|| None::<f64>);
    let search_query = use_state(|| String::new());
    let selected_category = use_state(|| -1_i32);
    let drawer_open = use_state(|| false);

    let on_weight_input = {
        let weight = weight.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f64>() {
                weight.set(Some(v));
            } else {
                weight.set(None);
            }
        })
    };

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value().to_lowercase());
        })
    };

    let on_bump = {
        let weight = weight.clone();
        move |delta: f64| {
            let weight = weight.clone();
            Callback::from(move |_| {
                let current = (*weight).unwrap_or(0.0);
                let new_val = (current + delta).max(0.0);
                weight.set(if new_val > 0.0 { Some(new_val) } else { None });
            })
        }
    };

    let toggle_drawer = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_| {
            let current = *drawer_open;
            drawer_open.set(!current);
        })
    };

    let close_drawer = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_: MouseEvent| {
            drawer_open.set(false);
        })
    };

    let peds_toc_val = PEDS_TOC.with(|v| v.clone());
    let peds_data_val = PEDS_DATA.with(|v| v.clone());

    if peds_data_val.is_undefined() || peds_toc_val.is_undefined() {
        return html! {
            <div style="padding: 20px; color: red;">
                <h2>{"錯誤：找不到藥物資料 (data.js 載入失敗)"}</h2>
                <p>{"請確認 data.js 是否有正確載入。"}</p>
            </div>
        };
    }

    let toc_array = Array::from(&peds_toc_val);
    let data_array = Array::from(&peds_data_val);

    // Filter logic
    let q = (*search_query).clone();
    let searching = !q.trim().is_empty();
    let cat = *selected_category;

    let mut any_shown = false;
    
    html! {
        <>
            <header>
                <div class="hwrap">
                    <div class="row1">
                        <button id="menuBtn" class="menu" type="button" aria-expanded={if *drawer_open { "true" } else { "false" }} onclick={toggle_drawer}>{"☰"}</button>
                        <h1>{"兒科"}<span>{"常用藥"}</span></h1>
                        <div class="bwbox">
                            <label for="bw">{"體重"}</label>
                            <input id="bw" type="number" inputmode="decimal" step="0.5" min="0" max="120" placeholder="—" 
                                value={weight.map(|v| v.to_string()).unwrap_or_default()}
                                oninput={on_weight_input} />
                            <span class="unit">{"kg"}</span>
                        </div>
                        <div class="steps">
                            <button type="button" onclick={on_bump(-1.0)}>{"−"}</button>
                            <button type="button" onclick={on_bump(1.0)}>{"＋"}</button>
                        </div>
                        <input id="q" type="search" placeholder="搜尋藥名 / 症狀" oninput={on_search_input} value={(*search_query).clone()} />
                    </div>
                </div>
            </header>

            <div class="shell">
                <aside id="toc" class={if *drawer_open { "open" } else { "" }}>
                    <div class="tochd"><span>{"分類"}</span><button id="tocX" type="button" onclick={close_drawer.clone()}>{"✕"}</button></div>
                    <nav id="tocnav">
                        <button class={classes!("tocitem", if !searching && cat == -1 { "on" } else { "" })} onclick={
                            let selected_category = selected_category.clone();
                            let search_query = search_query.clone();
                            let close = close_drawer.clone();
                            Callback::from(move |e: MouseEvent| {
                                selected_category.set(-1);
                                search_query.set(String::new());
                                close.emit(e);
                            })
                        }>
                            <span class="tn">{"全部"}</span>
                        </button>
                        {
                            for (0..toc_array.length()).map(|i| {
                                let grp = toc_array.get(i);
                                let g = Reflect::get(&grp, &JsValue::from_str("g")).unwrap().as_string().unwrap_or_default();
                                let idx_arr = Array::from(&Reflect::get(&grp, &JsValue::from_str("idx")).unwrap());
                                html! {
                                    <>
                                        if !g.is_empty() {
                                            <div class="tocgrp">{g}</div>
                                        }
                                        {
                                            for (0..idx_arr.length()).map(|j| {
                                                let data_idx = idx_arr.get(j).as_f64().unwrap() as i32;
                                                let sec = data_array.get(data_idx as u32);
                                                let c = Reflect::get(&sec, &JsValue::from_str("c")).unwrap().as_string().unwrap_or_default();
                                                
                                                let selected_category = selected_category.clone();
                                                let search_query = search_query.clone();
                                                let close = close_drawer.clone();
                                                let onclick = Callback::from(move |e: MouseEvent| {
                                                    selected_category.set(data_idx);
                                                    search_query.set(String::new());
                                                    close.emit(e);
                                                });
                                                
                                                html! {
                                                    <button class={classes!("tocitem", if !searching && cat == data_idx { "on" } else { "" })} onclick={onclick}>
                                                        <span class="tn">{c}</span>
                                                    </button>
                                                }
                                            })
                                        }
                                    </>
                                }
                            })
                        }
                    </nav>
                </aside>

                <main>
                    <div id="app">
                        {
                            for (0..data_array.length()).map(|i| {
                                let sec = data_array.get(i);
                                let c = Reflect::get(&sec, &JsValue::from_str("c")).unwrap().as_string().unwrap_or_default();
                                let k = Reflect::get(&sec, &JsValue::from_str("k")).unwrap().as_string().unwrap_or_default();
                                let d_arr = Array::from(&Reflect::get(&sec, &JsValue::from_str("d")).unwrap());

                                let mut sec_shown = 0;
                                let cards = html! {
                                    for (0..d_arr.length()).map(|j| {
                                        let drug = d_arr.get(j);
                                        let n = Reflect::get(&drug, &JsValue::from_str("n")).unwrap().as_string().unwrap_or_default();
                                        let rt = Reflect::get(&drug, &JsValue::from_str("rt")).ok().and_then(|v| v.as_string()).unwrap_or_default();
                                        let s = Reflect::get(&drug, &JsValue::from_str("s")).ok().and_then(|v| v.as_string()).unwrap_or_default();
                                        let w = Reflect::get(&drug, &JsValue::from_str("w")).ok().and_then(|v| v.as_string()).unwrap_or_default();
                                        let wm = Reflect::get(&drug, &JsValue::from_str("wm")).ok().and_then(|v| v.as_string()).unwrap_or_default();
                                        let f = Reflect::get(&drug, &JsValue::from_str("f")).ok().and_then(|v| v.as_string()).unwrap_or_default();
                                        
                                        let search_key = format!("{} {} {} {} {} {}", n, s, f, c, k, rt).to_lowercase();
                                        let search_hit = !searching || search_key.contains(&q);
                                        
                                        let cat_hit = searching || cat == -1 || cat == (i as i32);
                                        let hit = search_hit && cat_hit;
                                        
                                        if hit { sec_shown += 1; }

                                        let m_val = Reflect::get(&drug, &JsValue::from_str("m")).unwrap_or(JsValue::UNDEFINED);
                                        let m_arr = if m_val.is_undefined() || m_val.is_null() {
                                            Array::new()
                                        } else {
                                            Array::from(&m_val)
                                        };
                                        
                                        let r_val = Reflect::get(&drug, &JsValue::from_str("r")).unwrap_or(JsValue::UNDEFINED);
                                        let r_arr = if r_val.is_undefined() || r_val.is_null() {
                                            None
                                        } else {
                                            Some(Array::from(&r_val))
                                        };

                                        let mut calc_html = html! { <p class="nobw">{"↑ 輸入體重後自動計算"}</p> };
                                        let calc_val = Reflect::get(&drug, &JsValue::from_str("calc")).unwrap_or(JsValue::NULL);
                                        
                                        if calc_val.is_function() {
                                            if let Some(w) = *weight {
                                                let js_func = js_sys::Function::from(calc_val);
                                                if let Ok(res) = js_func.call1(&JsValue::NULL, &JsValue::from_f64(w)) {
                                                    if !res.is_null() && !res.is_undefined() {
                                                        let rows = Array::from(&res);
                                                        let mains = (0..rows.length()).filter(|&idx| {
                                                            let row = rows.get(idx);
                                                            let sub = Reflect::get(&row, &JsValue::from_str("sub"))
                                                                .ok()
                                                                .and_then(|v| v.as_bool())
                                                                .unwrap_or(false);
                                                            !sub
                                                        }).count();
                                                        
                                                        let mut seen = 0;
                                                        calc_html = html! {
                                                            for (0..rows.length()).map(|idx| {
                                                                let row = rows.get(idx);
                                                                let lbl = Reflect::get(&row, &JsValue::from_str("lbl")).unwrap_or(JsValue::UNDEFINED).as_string().unwrap_or_default();
                                                                let big = Reflect::get(&row, &JsValue::from_str("big")).unwrap_or(JsValue::UNDEFINED).as_string().unwrap_or_default();
                                                                let freq = Reflect::get(&row, &JsValue::from_str("freq")).unwrap_or(JsValue::UNDEFINED).as_string().unwrap_or_default();
                                                                let flag = Reflect::get(&row, &JsValue::from_str("flag")).unwrap_or(JsValue::UNDEFINED).as_string().filter(|s| !s.is_empty());
                                                                let sub = Reflect::get(&row, &JsValue::from_str("sub")).unwrap_or(JsValue::UNDEFINED).as_bool().unwrap_or(false);
                                                                
                                                                if sub {
                                                                    html! {
                                                                        <div class="dose sub">
                                                                            <span class="lbl">{lbl}</span>
                                                                            <span class="big">{safe_html(&big)}</span>
                                                                            if !freq.is_empty() { <span class="freq">{freq}</span> }
                                                                            if let Some(fg) = flag { <span class="flag cap">{format!("⚠ {}", fg)}</span> }
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    let mut cls = "dose".to_string();
                                                                    if mains > 2 && seen > 0 { cls.push_str(" sm"); }
                                                                    seen += 1;
                                                                    html! {
                                                                        <div class={cls}>
                                                                            <span class="lbl">{lbl}</span>
                                                                            <div class="big">{safe_html(&big)}</div>
                                                                            if !freq.is_empty() { <div class="freq">{freq}</div> }
                                                                            if let Some(fg) = flag { <span class="flag cap">{format!("⚠ {}", fg)}</span> }
                                                                        </div>
                                                                    }
                                                                }
                                                            })
                                                        };
                                                    }
                                                }
                                            }
                                        } else {
                                            calc_html = html! {};
                                        }

                                        let is_inj = rt == "針劑";
                                        let hl = n.starts_with('★');
                                        let card_cls = classes!("card", if !hit { "hide" } else { "" }, if is_inj { "inj" } else { "" }, if hl { "hl" } else { "" });

                                        html! {
                                            <article class={card_cls}>
                                                <p class="dn">{n.clone()} if !rt.is_empty() { <span class={classes!("rt", if is_inj { "inj" } else { "" })}>{rt.clone()}</span> }</p>
                                                if !s.is_empty() { <p class="dsub">{safe_html(&s)}</p> }
                                                if !w.is_empty() { <p class="warnbox">{safe_html(&format!("⚠ {}", w))}</p> }
                                                if !wm.is_empty() { <p class="warnbox mild">{safe_html(&format!("※ {}", wm))}</p> }
                                                
                                                <div class="out">{calc_html}</div>
                                                
                                                <div class="meta">
                                                    <div><span class="f">{f}</span></div>
                                                    {
                                                        for (0..m_arr.length()).map(|idx| {
                                                            let m_str = m_arr.get(idx).as_string().unwrap_or_default();
                                                            html! { <div>{safe_html(&m_str)}</div> }
                                                        })
                                                    }
                                                    if let Some(refs) = r_arr {
                                                        <div class="refs">{"來源："}
                                                        {
                                                            for (0..refs.length()).map(|idx| {
                                                                let ref_item = Array::from(&refs.get(idx));
                                                                let ref_name = ref_item.get(0).as_string().unwrap_or_default();
                                                                let ref_link = ref_item.get(1).as_string().unwrap_or_default();
                                                                html! {
                                                                    <>
                                                                        if idx > 0 { {" ・ "} }
                                                                        <a href={ref_link} target="_blank" rel="noopener noreferrer">{ref_name}</a>
                                                                    </>
                                                                }
                                                            })
                                                        }
                                                        </div>
                                                    }
                                                </div>
                                            </article>
                                        }
                                    })
                                };

                                if sec_shown > 0 { any_shown = true; }

                                html! {
                                    <section class={classes!("sec", if sec_shown == 0 { "hide" } else { "" })}>
                                        <h2>{c}</h2>
                                        <div class="grid">
                                            {cards}
                                        </div>
                                    </section>
                                }
                            })
                        }
                    </div>
                    if !any_shown {
                        <div id="empty" class="hide">{"找不到符合的藥物"}</div>
                    }
                </main>
            </div>
            if *drawer_open {
                <div id="backdrop" class="on" onclick={close_drawer}></div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
