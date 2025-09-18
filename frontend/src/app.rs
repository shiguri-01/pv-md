use api::get_root_dir;
use leptos::prelude::*;
use thaw::ConfigProvider;

#[component]
pub fn App() -> impl IntoView {
    let dir = LocalResource::new(|| async move { get_root_dir().await });
    view! {
        <ConfigProvider>
            <div>
                <Suspense fallback=|| {
                    view! { <h1>"Loading..."</h1> }
                }>
                    {move || {
                        dir.get()
                            .map(|result| match result {
                                Ok(dir) => view! { <h1>{dir}</h1> }.into_view(),
                                Err(e) => view! { <h1>{format!("Error: {}", e)}</h1> }.into_view(),
                            })
                    }}
                </Suspense>
                <p>"Hello, world!"</p>
            </div>
        </ConfigProvider>
    }
}
