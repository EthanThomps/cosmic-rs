use leptos::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <div class="p-5 text-4xl">    
                // A gradient from purple to orange?                    
                <figcaption>"A Cosmic Web Tool for Nasa Hunch"</figcaption>
            </div>
        </div>
    }
}