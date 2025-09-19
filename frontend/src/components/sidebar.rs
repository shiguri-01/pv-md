use api::{NavTreeDto, get_nav_tree};
use leptos::prelude::*;
use leptos_router::components::A;
use thaw::{Tree, TreeItem, TreeItemLayout, TreeItemType};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
      <div class="sidebar">
        <NavTreeSection />
      </div>
    }
}

#[component]
fn NavTreeSection() -> impl IntoView {
    let nav_trees = LocalResource::new(|| async move { get_nav_tree().await });
    view! {
      <div>
        <ErrorBoundary fallback=|_| view! { <p>"Error loading navigation"</p> }>
          <Suspense fallback=|| {
            view! { <p>"Loading..."</p> }
          }>
            {move || {
              nav_trees
                .get()
                .map(|result| { result.map(|nav_trees| view! { <NavTreeList nav_trees /> }) })
            }}
          </Suspense>
        </ErrorBoundary>
      </div>
    }
}

#[component]
fn NavTreeList(#[prop(into)] nav_trees: Signal<Vec<NavTreeDto>>) -> impl IntoView {
    view! {
      <Tree>
        <For
          each=move || nav_trees.get()
          key=get_nav_tree_key
          children=|child| view! { <NavTreeItem nav_tree=child /> }
        />
      </Tree>
    }
}

#[component]
fn NavTreeItem(#[prop(into)] nav_tree: Signal<NavTreeDto>) -> impl IntoView {
    match nav_tree.get() {
        NavTreeDto::File { name, path } => view! { <FileItem name path /> }.into_any(),
        NavTreeDto::Dir { name, children, .. } => {
            view! { <DirectoryItem name children /> }.into_any()
        }
    }
}

#[component]
fn FileItem(name: String, path: String) -> impl IntoView {
    let link = format!("/docs/{}", &path);
    view! {
      <TreeItem item_type=TreeItemType::Leaf>
        <TreeItemLayout>
          <A href=link>{name}</A>
        </TreeItemLayout>
      </TreeItem>
    }
}

#[component]
fn DirectoryItem(name: String, children: Vec<NavTreeDto>) -> impl IntoView {
    view! {
      <TreeItem item_type=TreeItemType::Branch>
        <TreeItemLayout>{name}</TreeItemLayout>
        <NavTreeList nav_trees=children />
      </TreeItem>
    }
}

fn get_nav_tree_key(nav_tree: &NavTreeDto) -> String {
    match nav_tree {
        NavTreeDto::File { path, .. } | NavTreeDto::Dir { path, .. } => path.clone(),
    }
}
