use crate::table::TableComponent;
use std::rc::Rc;
use dominator::{Dom, html, events, with_node, clone};
use crate::item::Item;
use web_sys::HtmlSelectElement;


#[derive(Debug)]
pub struct App {
    items: Vec<Item>,
    entries: Vec<String>,
    selected_entry: String,
}

impl App {
    pub async fn new() -> Rc<Self> {
        let items = crate::db_interface::get().await;
        let entries = crate::db_interface::get_entries().await;
        let selected_entry = entries[0].clone();
        Rc::new(Self {
            items,
            entries,
            selected_entry
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("main", {
            .children(&mut [
                html!("select" => HtmlSelectElement, {
                    .with_node!(elem => {
                        .event(clone!(elem => move |_:events::Change| {
                            let selected_entry: String = elem.value();
                            crate::utils::log(&selected_entry);
                        }))
                    })
                    .children(
                        app.entries.iter().map(|e| {
                            html!("option", {
                                .property("text", e.to_string())
                                .property("value", e.to_string())
                                .property("selected", e == &app.selected_entry)
                            })
                        })
                    )
                }),
                html!("div", {
                    .class("icon-button")
                    .class("select-columns")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/select-columns-icon.png")
                            }))
                            .event(|_event: events::Click| {
                                crate::utils::log("select columns clicked");
                            })
                        }),
                        html!("span", {
                            .text("Select columns to display")
                        }),
                    ])
                }),
                html!("div", {
                    .class("icon-button")
                    .class("add-text")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/add-icon.png")
                            }))
                            .event(|_event: events::Click| {
                                crate::utils::log("add text clicked");
                            })
                        }),
                        html!("span", {
                            .text("Add a text")
                        }),
                    ])
                }),
                TableComponent::render(TableComponent::new(&app.items)),
            ])
        })
    }
}
