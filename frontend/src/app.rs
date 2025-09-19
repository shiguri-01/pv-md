use api::get_root_dir;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

use crate::components::sidebar::Sidebar;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <ConfigProvider>
        <Router>
          <Sidebar />
          <Routes fallback=|| view! { <p>"Not Found"</p> }>
            <Route path=path!("/") view=Home />
            // TODO: ページを作成する
            <Route path=path!("/docs/*file_path") view=Home />
          </Routes>
        </Router>
      </ConfigProvider>
    }
}

#[component]
fn Home() -> impl IntoView {
    let dir = LocalResource::new(|| async move { get_root_dir().await });

    view! {
      <Suspense fallback=|| {
        view! { <h1>"Loading..."</h1> }
      }>
        {move || {
          dir
            .get()
            .map(|result| match result {
              Ok(dir) => view! { <h1>{dir}</h1> }.into_view(),
              Err(e) => view! { <h1>{format!("Error: {}", e)}</h1> }.into_view(),
            })
        }}
      </Suspense>
      <p>"Hello, world!"</p>
    }
}
