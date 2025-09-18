use leptos::prelude::*;
use thaw::ConfigProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <div>
                <p>"Hello, world!"</p>
            </div>
        </ConfigProvider>
    }
}
