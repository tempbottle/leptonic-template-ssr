use leptonic::components::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = RwSignal::new(0).split();

    let on_press = move |_| {
        log!("Button press.");
        set_count.update(|c| *c += 1);
    };
    view! {
        <div style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <h2>"Welcome to Leptonic"</h2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_press=on_press>"Increase"</Button>
        </div>
    }
}
