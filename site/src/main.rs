mod app;
mod pages;
mod routing;
// mod theme;
use leptos::*;
use crate::app::App;


fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
