use sycamore::prelude::*;
use unishorten::{StringShortener};
use unidecode::unidecode;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let Shortener = StringShortener::new();
    let input = create_signal(cx, String::new());
    let use_chars = create_signal(cx, String::new());
    let out = 
        create_memo(cx, move || {
            match use_chars.get().as_str() {
                "chars" => Shortener.shorten_by_chars(&input.get()),
                "bytes" => Shortener.shorten_by_bytes(&input.get()),
                _ => Shortener.shorten_by_chars(&input.get())
            }
        });

    view! { cx,
        div {
            div(class="container mb-2em") {
                h1 { "Unicode String Shortener" }

                """
                This program will replace sets of characters with a single character to shorten a string. Note that the output may actually be larger in bytes than the input, because Unicode characters vary in size.
                """
            }

            div(class="container mb-2em") {
                div(class="container") {
                    div(class="w-50 half-div") {
                        textarea(id="input", placeholder="input (try \"aether\")", bind:value=input)
                        span { (input.get().chars().count()) " characters, " (input.get().len()) " bytes" }
                    }

                    div(class="w-50 half-div") {
                        textarea(id="output", readonly=true, placeholder="output") { (out.get()) }
                        span { (out.get().chars().count()) " characters, " (out.get().len()) " bytes" }
                        button(id="copy-output", onclick="document.getElementById('output').select(); document.execCommand('copy');") { "Copy Output" }
                    }
                }

                details(id="more-options") {
                    summary { "Options" }
                    div(id="options-div") {
                        "Reduce the number of "
                        select(style="display: inline;", bind:value=use_chars) {
                            option(value="chars") { "Characters" }
                            option(value="bytes") { "Bytes" }
                        }
                        "."
                    }

                    label(for="computer-sees") { "A computer would transliterate this as" }
                        input(id="computer-sees", readonly=true, style="width: 100%; box-sizing: border-box;", placeholder="output", value=unidecode(&out.get())) {}
                    
                }
            }

            footer {
                small {
                "Created by "
                a(href="https://noguera.dev/"){"Michael Noguera"}
                ". See source code on "
                a(href="https://github.com/michaelnoguera/unicode-string-shortener"){"Github"} "."

                " All processing done client-side."
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