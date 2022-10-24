// use ybc::prelude::*;
use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlElement, MouseEvent};
use yew::{
    html,
    Callback, TargetCast, 
};
 

#[function_component(Counter)]
pub fn counter() -> Html {
    let data = use_state(|| Option::<u32>::None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::post("/api/counter").send().await.expect("request failed");

                    if !resp.ok() {
                        tracing::error!(
                            "Error fetching data {} ({})",
                            resp.status(),
                            resp.status_text()
                        );
                        return;
                    }

                    let content = match resp.text().await {
                        Err(err) => {
                            tracing::error!("Error fetching data {err}");
                            return;
                        }
                        Ok(content) => content,
                    };

                    let count = match content.parse() {
                        Err(err) => {
                            tracing::error!("Data is not a number: {err}");
                            return;
                        }
                        Ok(count) => count,
                    };

                    data.set(Some(count));
                });
            }

            || {}
        });
    }

    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
            console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        }
    });

    html! {
        <div>
            <div id="mousemoveme" {onmousemove}>{"move"}</div>
            <div>{data.map(|d|d.to_string()).unwrap_or_default()}</div>
        </div>
    }
}
