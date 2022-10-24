mod components;

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
// use ybc::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{console, HtmlElement, MouseEvent};

use crate::components::counter::*;

// use wasm_bindgen::prelude::*;
// use web_sys::window;

// #[wasm_bindgen(module = "/public/glue.js")]
// extern "C" {
//     #[wasm_bindgen(js_name = invokeHello, catch)]
//     pub async fn hello(name: String) -> Result<JsValue, JsValue>;
// }

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// #[function_component(Counter)]
// fn counter() -> Html {
//     let data = use_state(|| Option::<u32>::None);
//     {
//         let data = data.clone();
//         use_effect(move || {
//             if data.is_none() {
//                 spawn_local(async move {
//                     let resp = Request::post("/api/counter").send().await.unwrap();

//                     if !resp.ok() {
//                         tracing::error!(
//                             "Error fetching data {} ({})",
//                             resp.status(),
//                             resp.status_text()
//                         );
//                         return;
//                     }

//                     let content = match resp.text().await {
//                         Err(err) => {
//                             tracing::error!("Error fetching data {err}");
//                             return;
//                         }
//                         Ok(content) => content,
//                     };

//                     let count = match content.parse() {
//                         Err(err) => {
//                             tracing::error!("Data is not a number: {err}");
//                             return;
//                         }
//                         Ok(count) => count,
//                     };

//                     data.set(Some(count));
//                 });
//             }

//             || {}
//         });
//     }

//     html! {
//         <div>
//             <div>{data.map(|d|d.to_string()).unwrap_or_default()}</div>
//         </div>
//     }
// }

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Counter => html! { <Counter /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    yew::start_app::<App>();
}
