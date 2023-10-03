use crate::routing::RouterDirector;
use crate::routing::RouterTraffic;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
/// The actuall application belongs here
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <div>
            <Router>
                <RouterDirector/>
                <RouterTraffic/>
            </Router>
        </div>
    }
}
