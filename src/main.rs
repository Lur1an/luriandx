#![allow(non_snake_case)]

use dioxus::prelude::*;
use lumen_blocks::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// LumenBlocksAccordion component
#[component]
fn LumenBlocksAccordion() -> Element {
    rsx! {
        div { class: "p-4 max-w-3xl mx-auto",
            h2 { class: "text-2xl font-bold mb-4", "About Lumen Blocks" }

            Accordion {
                allow_multiple_open: true,
                horizontal: false,

                AccordionItem {
                    index: 0,
                    AccordionTrigger { "What is Lumen Blocks?" }
                    AccordionContent {
                        p { class: "mb-2",
                            "Lumen Blocks is an ARIA-accessible, styled, opinionated component library for Dioxus inspired by the shadcn UI project, and built on top of the Dioxus Primitives unstyled components library."
                        }
                        p {
                            "It provides ready-to-use UI components that are both beautiful and accessible."
                        }
                    }
                }

                AccordionItem {
                    index: 1,
                    AccordionTrigger { "Getting Started" }
                    AccordionContent {
                        p { class: "mb-2",
                            "This repository should have everything installed! You are ready to start using Lumen Blocks!"
                        }
                        p { class: "mt-2",
                            "For detailed installation instructions, visit the ",
                            a {
                                class: "text-blue-500 underline",
                                href: "https://lumenblocks.dev/docs/0.1/installation/",
                                "Installation Guide"
                            }
                        }
                    }
                }

                AccordionItem {
                    index: 2,
                    AccordionTrigger { "Available Components" }
                    AccordionContent {
                        p { class: "mb-2",
                            "Lumen Blocks offers a variety of components including:"
                        }
                        ul { class: "list-disc pl-5 space-y-1",
                            li { "Accordion (like this one!)" }
                            li { "Buttons" }
                            li { "Hover Cards" }
                            li { "Side Sheets" }
                            li { "Form inputs" }
                            li { "And many more!" }
                        }
                        p { class: "mt-2",
                            "Check the ",
                            a {
                                class: "text-blue-500 underline",
                                href: "https://lumenblocks.dev",
                                "documentation"
                            },
                            " for a complete list of components and usage examples."
                        }
                    }
                }

                AccordionItem {
                    index: 3,
                    AccordionTrigger { "Project Status" }
                    AccordionContent {
                        p {
                            "Lumen Blocks is currently experimental. Expect bugs and breaking changes until a stable release is made. The project is actively developed and maintained."
                        }
                    }
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        LumenBlocksAccordion {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
