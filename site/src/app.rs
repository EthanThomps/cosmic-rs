use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provice_meta_context(cx);

    view! {cx,
        <Layout>
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                <Route path="/" view=move |cx| view! {cx, <Home/>} />
                <Route path="/about" view=move |cx| view! {cx, <About/>} />
                // <Route path=""/>
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
    let default_class = default_page_class();

    view! {cx,
        <div class=default_class.wrapper>
        {children(cx)}
        </div>
    }
}
