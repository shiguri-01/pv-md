use api::{NavTreeDto, get_nav_tree};
use leptos::prelude::*;
use leptos_router::components::A;
use thaw::{Tree, TreeItem, TreeItemLayout, TreeItemType};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <Navs />
        </div>
    }
}

#[component]
fn Navs() -> impl IntoView {
    let nav_trees = LocalResource::new(|| async move { get_nav_tree().await });
    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                nav_trees
                    .get()
                    .map(|result| match result {
                        Ok(nav_trees) => {
                            view! {
                                <Tree>
                                    <For
                                        each=move || { nav_trees.clone() }
                                        key=|child| match child {
                                            NavTreeDto::File { path, .. } => path.clone(),
                                            NavTreeDto::Dir { path, .. } => path.clone(),
                                        }
                                        children=|child| view! { <Nav nav_tree=child /> }
                                    />
                                </Tree>
                            }
                                .into_any()
                        }
                        Err(_) => view! { <p>"Error loading navigation"</p> }.into_any(),
                    })
            }}

        </Suspense>
    }
}

#[component]
fn Nav(nav_tree: NavTreeDto) -> impl IntoView {
    match nav_tree {
        NavTreeDto::File { name, path } => view! {
            <TreeItem item_type=TreeItemType::Leaf>
                <TreeItemLayout>
                    <A href=format!("/docs/{}", &path)>{name}</A>
                </TreeItemLayout>
            </TreeItem>
        },
        NavTreeDto::Dir { name, children, .. } => view! {
            <TreeItem item_type=TreeItemType::Branch>
                <TreeItemLayout>{name}</TreeItemLayout>
                <Tree>
                    <For
                        each=move || { children.clone() }
                        key=|child| match child {
                            NavTreeDto::File { path, .. } => path.clone(),
                            NavTreeDto::Dir { path, .. } => path.clone(),
                        }
                        children=|child| view! { <Nav nav_tree=child /> }
                    />
                </Tree>
            </TreeItem>
        },
    }
}
