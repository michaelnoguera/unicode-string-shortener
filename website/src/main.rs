use sycamore::prelude::*;
use unishorten::{StringShortener};

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let Shortener = StringShortener::new();
    let input = create_signal(cx, String::new());
    let out = 
        create_memo(cx, move || {
            Shortener.shorten_by_chars(&input.get())
        });

    view! { cx,
        div {
            h1 { "Unicode String Shortener" }

            input(placeholder="input", bind:value=input)
            
            div { (out.get()) }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| 
        view! { cx, App {} });
}