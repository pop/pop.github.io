use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::config::WebampTrack;

// JS bridge: the `index.html` `<script type="module">` imports Webamp from the
// CDN and stashes it on `window.Webamp`. We reach it via wasm-bindgen.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = Webamp)]
    type JsWebamp;

    #[wasm_bindgen(constructor, js_class = "Webamp", js_namespace = window)]
    fn new(options: &JsValue) -> JsWebamp;

    #[wasm_bindgen(method, js_name = renderWhenReady)]
    fn render_when_ready(this: &JsWebamp, target: &web_sys::Element) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = close)]
    fn close(this: &JsWebamp);
}

// Options object matching Webamp's JS API. serde-wasm-bindgen serializes this
// to a plain JS object the constructor understands.
#[derive(Serialize)]
struct WebampOptions {
    #[serde(rename = "initialTracks")]
    initial_tracks: Vec<WebampInitialTrack>,
    #[serde(rename = "initialSkin", skip_serializing_if = "Option::is_none")]
    initial_skin: Option<WebampSkin>,
    #[serde(rename = "initialWindowLayout", skip_serializing_if = "Option::is_none")]
    initial_window_layout: Option<WebampWindowLayout>,
}

#[derive(Serialize)]
struct WebampWindowLayout {
    main: WebampWindowPosition,
    equalizer: WebampWindowPosition,
    playlist: WebampWindowPosition,
}

#[derive(Serialize)]
struct WebampWindowPosition {
    position: WebampPoint,
}

// Webamp uses CSS-style `top`/`left` keys here, not `x`/`y`. Wrong keys are
// silently ignored.
#[derive(Serialize)]
struct WebampPoint {
    top: i32,
    left: i32,
}

#[derive(Serialize)]
struct WebampInitialTrack {
    url: String,
    #[serde(rename = "metaData")]
    meta_data: WebampMeta,
}

#[derive(Serialize)]
struct WebampMeta {
    artist: String,
    title: String,
}

#[derive(Serialize)]
struct WebampSkin {
    url: String,
}

#[derive(Properties, PartialEq)]
pub struct WebampProps {
    pub tracks: Vec<WebampTrack>,
    #[prop_or_default]
    pub skin_url: Option<String>,
    #[prop_or(20)]
    pub top: i32,
}

#[function_component(Webamp)]
pub fn webamp(props: &WebampProps) -> Html {
    let mount_ref = use_node_ref();

    {
        let mount_ref = mount_ref.clone();
        let tracks = props.tracks.clone();
        let skin_url = props.skin_url.clone();
        let top = props.top;
        use_effect_with((tracks, skin_url, top), move |(tracks, skin_url, top)| {
            // Shared mutable handle so both the init path and the cleanup closure
            // can reach the constructed Webamp instance.
            let instance: std::rc::Rc<std::cell::RefCell<Option<JsWebamp>>> =
                std::rc::Rc::new(std::cell::RefCell::new(None));
            // Keep any pending "waiting for Webamp" listener/timer alive.
            let pending_listener: std::rc::Rc<std::cell::RefCell<Option<EventListener>>> =
                std::rc::Rc::new(std::cell::RefCell::new(None));
            let pending_timeout: std::rc::Rc<std::cell::RefCell<Option<Timeout>>> =
                std::rc::Rc::new(std::cell::RefCell::new(None));

            let tracks = tracks.clone();
            let skin_url = skin_url.clone();
            let top = *top;
            let mount_ref = mount_ref.clone();

            let init = {
                let instance = instance.clone();
                let pending_listener = pending_listener.clone();
                let pending_timeout = pending_timeout.clone();
                move || {
                    // Bail if the container isn't mounted yet.
                    let Some(target) = mount_ref.cast::<web_sys::Element>() else {
                        return;
                    };

                    // Center horizontally and stack main / equalizer / playlist
                    // from the configured top offset. Webamp windows are 275px
                    // wide; main + equalizer are 116px tall each.
                    let layout = web_sys::window().and_then(|w| {
                        w.inner_width().ok().and_then(|v| v.as_f64()).map(|width| {
                            let left = ((width as i32 - 275) / 2).max(0);
                            WebampWindowLayout {
                                main: WebampWindowPosition {
                                    position: WebampPoint { top, left },
                                },
                                equalizer: WebampWindowPosition {
                                    position: WebampPoint { top: top + 116, left },
                                },
                                playlist: WebampWindowPosition {
                                    position: WebampPoint { top: top + 232, left },
                                },
                            }
                        })
                    });

                    let opts = WebampOptions {
                        initial_tracks: tracks
                            .iter()
                            .map(|t| WebampInitialTrack {
                                url: t.url.clone(),
                                meta_data: WebampMeta {
                                    artist: t.artist.clone(),
                                    title: t.title.clone(),
                                },
                            })
                            .collect(),
                        initial_skin: skin_url
                            .clone()
                            .map(|url| WebampSkin { url }),
                        initial_window_layout: layout,
                    };

                    let js_opts = match serde_wasm_bindgen::to_value(&opts) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("webamp: failed to serialize options: {e:?}");
                            return;
                        }
                    };

                    let wa = JsWebamp::new(&js_opts);
                    let _ = wa.render_when_ready(&target);
                    *instance.borrow_mut() = Some(wa);
                    // Clear any pending retry/listener now that we're up.
                    *pending_listener.borrow_mut() = None;
                    *pending_timeout.borrow_mut() = None;
                }
            };

            // The CDN module import is async. If `window.Webamp` isn't defined
            // yet, wait for the `webamp-ready` event dispatched by index.html.
            // As a belt-and-suspenders fallback, also poll briefly via Timeout.
            let webamp_defined = js_sys::Reflect::get(
                &web_sys::window().unwrap(),
                &JsValue::from_str("Webamp"),
            )
            .map(|v| !v.is_undefined() && !v.is_null())
            .unwrap_or(false);

            if webamp_defined {
                init();
            } else {
                let window = web_sys::window().unwrap();
                let init_listener = {
                    let init = init.clone();
                    move |_: &web_sys::Event| init()
                };
                let listener = EventListener::new(&window, "webamp-ready", init_listener);
                *pending_listener.borrow_mut() = Some(listener);

                // Fallback poll in case the event fired before we subscribed.
                let pending_timeout_cl = pending_timeout.clone();
                let init_cl = init.clone();
                let t = Timeout::new(100, move || {
                    let defined = js_sys::Reflect::get(
                        &web_sys::window().unwrap(),
                        &JsValue::from_str("Webamp"),
                    )
                    .map(|v| !v.is_undefined() && !v.is_null())
                    .unwrap_or(false);
                    if defined {
                        init_cl();
                    }
                    // drop our handle to this Timeout so RefCell is clear
                    let _ = pending_timeout_cl;
                });
                *pending_timeout.borrow_mut() = Some(t);
            }

            move || {
                // Tear down any pending listener/timer first so they can't fire
                // after the component is gone.
                *pending_listener.borrow_mut() = None;
                *pending_timeout.borrow_mut() = None;
                if let Some(wa) = instance.borrow_mut().take() {
                    wa.close();
                }
            }
        });
    }

    html! {
        <div ref={mount_ref} id="webamp-mount"></div>
    }
}
