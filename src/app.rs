use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::*;
use leptos_router::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::welcome::Welcome;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <Meta name="charset" content="UTF-8" />
                <Meta name="description" content="Leptonic SSR template" />
                <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <Meta name="theme-color" content="#8856e6" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css" />
                <Stylesheet href="/Roboto.css" />
                <Title text="Leptonic SSR template" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}


#[component]
pub fn App() -> impl IntoView {
    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors /> }
                }>
                    <Route path=path!("") view=|| view! { <Welcome /> } />
                </Routes>
            </Router>
        </Root>
    }
}