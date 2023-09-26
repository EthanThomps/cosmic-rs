use std::ops::DerefMut;

use leptos::*;

fn main() {   
    mount_to_body(|| view! { <App/> })
}


#[component]
fn App() -> impl IntoView {
   view! {
    <SimpleCounter/>
   }
}

#[component]
fn SimpleCounter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);

    view! {
        <button 
            on:click=move |_| {set_count.update(|count| *count += 1)}
            class:red=move || {count.get() % 2 == 1}
        >
            "Click Me "
            {count}
        </button>
        <progress max="50" value=count/>
        <h1 
            style="position: absolute"
        >
           "Hi Mom" 
        </h1>
    }
}