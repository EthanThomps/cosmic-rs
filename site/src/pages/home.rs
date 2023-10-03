use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <div class="p-5 text-2xl underline font-bold">
                <figcaption>"The home page"</figcaption>
            </div>
        </div>
    }
}