use crate::pages::{about::About, home::Home};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
// Icons
// https://carlosted.github.io/icondata/

#[component]
/// Directs routes through traffic with layout
pub(crate) fn RouterTraffic(cx: Scope) -> impl IntoView {
    view! {cx,
        <main class="bg-slate-500">
            <Routes>
                <Route path="" view=Home/>
                <Route path="about" view=About/>
            </Routes>
        </main>
    }
}

#[component]
pub(crate) fn MenuOpen(cx: Scope) -> impl IntoView {
    view! {cx,
            <Icon icon=Icon::from(AiIcon::AiMenuOutlined)/>
    }
}

#[component]
pub(crate) fn MenuClose(cx: Scope) -> impl IntoView {
    view! {cx,
            <div class="text-4xl">
                <Icon icon=Icon::from(AiIcon::AiCloseOutlined)/>
            </div>
    }
}

#[component]
pub(crate) fn CosmicLogo(cx: Scope) -> impl IntoView {
    view! {cx,
            <div class="text-4xl">
                <Icon icon=Icon::from(BsIcon::BsRocketTakeoff)/>
            </div>
    }
}

#[component]
/// links to each route
pub(crate) fn RouterDirector(cx: Scope) -> impl IntoView {
    // Next! get surrealdb to work
    view! {cx,
        <GlobalNavbar/>
    }
}

#[component]
// https://tailwindcss.com/docs/background-color
pub(crate) fn GlobalNavbar(cx: Scope) -> impl IntoView {
    view! {cx,
        <nav class="bg-gray-100">
            <div class="p-2 xl:max-w-6xl mx-auto border border-red-400">
                <div class="flex space-x-4">
                    // Logo
                    <div class="flex mx-2 px-2">
                        <CosmicLogo/>
                        <p class="text-2xl mx-2">Cosmicli</p>
                    </div>

                    // Primary
                    <div>
                        <a href="/">Home</a>
                        <a href="about">About</a>
                    </div>

                    // Secondary
                    <div>
                        secondary
                    </div>
                </div>
            </div>
        </nav>
    }
}
