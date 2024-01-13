use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HelloFrontend /> },
        Route::HelloServer => html! { <HelloServer /> },
    }
}

#[function_component(HelloFrontend)]
fn hello_frontend() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::HelloServer));

    html! {
        <>
            <h1>{ "Hello from the Frontend!" }</h1>
            <button {onclick}>{ "Response" }</button>
            <button><a href="/hello">{ "Backend" }</a></button>
        </>
    }
}

#[function_component(HelloServer)]
fn hello_backend() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    match data.as_ref() {
        None => {
            html! {
            <>
                <div>{"No server response"}</div>
                <button {onclick}>{ "Go Home" }</button>
            </>
            }
        }
        Some(Ok(data)) => {
            html! {
            <>
                <div>{"Got server response: "}{data}</div>
                <button {onclick}>{ "Go Home" }</button>
            </>
            }
        }
        Some(Err(err)) => {
            html! {
            <>
                <div>{"Error requesting data from server: "}{err}</div>
                <button {onclick}>{ "Go Home" }</button>
            </>
            }
        }
    }
}
