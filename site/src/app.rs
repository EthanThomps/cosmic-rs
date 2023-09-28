use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::about::About;
use crate::pages::home::Home;

// This entire App Component is incorrect, follow from tutorial
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <div>
            <Router>
                <nav>
                    <a href="about">"About"</a>
                    <a href="/">"Home"</a>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="about" view=About/>
                    </Routes>
                </main>
            </Router>
        </div>    
    }
}
