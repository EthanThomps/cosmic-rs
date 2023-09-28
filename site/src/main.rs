mod app;
mod pages;
// mod theme;
use leptos::*;
use crate::app::App;


fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    // console_error_panic_hook::set_once();

    // log!("Basic CSR mode template");

    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
