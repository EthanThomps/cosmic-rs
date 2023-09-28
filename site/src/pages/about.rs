use leptos::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <h1>"About Page"</h1>
            <p class="1xl bg-white">"Does this work?"</p>
        </div>
    }
}