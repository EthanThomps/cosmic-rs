use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="text-2xl underline font-bold">
            <h1>"Home Page"</h1>
        </div>
    }
}