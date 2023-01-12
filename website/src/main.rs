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

            div {
                """
                This program will replace sets of characters with a single character to shorten a string. Note that the output is likely not machine readable. Also, because Unicode characters vary in size, it may actually be bigger in bytes than the input.
                """
            }

            br{}
            
            div { 
                input(placeholder="input (try \"aether\")", bind:value=input)
                (input.get()) " (" (input.get().chars().count()) " characters)" " -> " (out.get()) " (" (out.get().chars().count()) " characters)"
            }

            br{}

            footer {
                small {
                "Created by "
                a(href="https://noguera.dev/"){"Michael Noguera"}
                ". See source code on "
                a(href="https://github.com/michaelnoguera/unicode-string-shortener"){"Github"} "."
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| 
        view! { cx, App {} });
}