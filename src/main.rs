use dioxus::prelude::*;

fn main() {
    let config = server_only! {
        let path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("public");

        let render_cfg = IncrementalRendererConfig::new()
            .static_dir(path)
            .clear_cache(false);

        ServeConfig::builder()
            .incremental(render_cfg)
            .enable_out_of_order_streaming();
    };

    LaunchBuilder::new()
        .with_cfg(config)
        .launch(|| rsx! {
            Router::<Route> {}
        })
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Index {},
}

#[component]
fn Index() -> Element {
    rsx! {
        p { "This site is a work in progress." }
    }
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let routes = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect();

    Ok(routes)
}
