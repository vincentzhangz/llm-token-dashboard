mod models;
mod pages;
mod utils;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use pages::{HomePage, InfoPage, ModelPriceListPage};
use thaw::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Router>
                <Routes fallback=|| "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("list-model-prices") view=ModelPriceListPage />
                    <Route path=StaticSegment("info") view=InfoPage />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
