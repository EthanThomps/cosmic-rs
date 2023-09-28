use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::about::About;
use crate::pages::home::Home;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <Layout>
            <Router>
                <Route path="/" view=move |cx| view! {cx, <Home/>} />
                <Route path="/about" view=move |cx| view! {cx, <About/>} />
                // <Route path="*" view=move |cx| view! {cx, <Error/>} />
            </Router>
        </Layout>
    }
}

// Not mine: wrapper test
#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <LayoutWrapper>
            {children(cx)}
        </LayoutWrapper>
    }
}

#[component]
fn LayoutWrapper(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="bg-slate-800">
        {children(cx)}
        </div>
    }
}
