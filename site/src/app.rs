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
                // Title
                <RouteDirector/>
                // <div class="bg-slate-700">
                //     <h1 href="/" class="mx-2 text-center">"Cosmic Web Tool"</h1>        
                //     // Page Links
                //     <nav class="bg-slate-700 text-center">
                //         <a href="/" class="mx-2 text-xs ">"Home"</a>
                //         <a href="about" class="mx-2 text-xs">"About"</a>
                //     </nav>
                // </div>
                
                // Pages
                <main class="bg-slate-500">
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="about" view=About/>
                    </Routes>
                </main>
            </Router>
        </div>    
    }
}

#[component]
fn RouteDirector(cx: Scope) -> impl IntoView {

    view! {cx,
        <div class="bg-slate-700">
            <h1 href="/" class="mx-2 text-center">"Cosmic Web Tool"</h1>        
            // Page Links
            <nav class="bg-slate-700 text-center">
                <a href="/" class="mx-2 text-xs ">"Home"</a>
                <a href="about" class="mx-2 text-xs">"About"</a>
            </nav>
        </div>
    }
}