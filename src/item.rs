use web_sys::HtmlSelectElement;
use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events, with_node};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumString, Display, EnumIter};
use crate::table::TableComponent;


#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq)]
pub enum ItemKind {
    Heading,
    Subheading,
    Button,
    Instruction,
    Toggle,
    Warning,
    Feedback,
    #[strum(serialize = "Help Text")]
    HelpText,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq)]
pub enum ItemStatus {
    Approved,
    Discuss,
    #[strum(serialize = "On Hold")]
    OnHold,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub db_id: i32,
    pub id: String,
    pub section: String,
    pub item_kind: ItemKind,
    // maybe not the best idea to hard code the languages 
    pub english: String,
    pub status: ItemStatus,
    pub zeplin_reference: String,
    pub comments: String,
    pub in_app: bool,
    pub in_element: bool,
    pub in_mock: bool,
}


#[derive(Clone)]
pub struct ItemComponent {

}

impl ItemComponent {
    pub fn render(item: Rc<Mutable<Item>>, table_state: Rc<TableComponent>) -> Dom {
        let item_ref = item.lock_ref();
        html!("tr", {
            .children(&mut [
                html!("td", {
                    .child(html!("input", {
                        .property("value", &item_ref.id)
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.id = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .property("value", &item_ref.section)
                        .attribute("list", "ice-cream-flavors")
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.section = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("select" => HtmlSelectElement, {
                        .with_node!(elem => {
                            .event(clone!(item => move |_event: events::Change| {
                                let value: String = elem.value();
                                let mut item = item.lock_mut();
                                item.item_kind = ItemKind::from_str(&value).unwrap_throw();
                            }))
                        })
                        .children(
                            ItemKind::iter().map(|o| {
                                html!("option", {
                                    .property("text", o.to_string())
                                    .property("value", o.to_string())
                                    .property("selected", o == item_ref.item_kind)
                                })
                            })
                        )
                    }))
                }),
                html!("td", {
                    .child(html!("textarea", {
                        .text(&item_ref.english)
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.english = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("select" => HtmlSelectElement, {
                        .with_node!(elem => {
                            .event(clone!(item => move |_event: events::Change| {
                                let value: String = elem.value();
                                let mut item = item.lock_mut();
                                item.status = ItemStatus::from_str(&value).unwrap_throw();
                            }))
                        })
                        .children(
                            ItemStatus::iter().map(|o| {
                                html!("option", {
                                    .property("text", o.to_string())
                                    .property("value", o.to_string())
                                    .property("selected", o == item_ref.status)
                                })
                            })
                        )
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .property("value", &item_ref.zeplin_reference)
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.zeplin_reference = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .property("value", &item_ref.comments)
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.comments = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", item_ref.in_app)
                        .event(clone!(item => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.in_app = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", item_ref.in_element)
                        .event(clone!(item => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.in_element = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", item_ref.in_mock)
                        .event(clone!(item => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.in_mock = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(
                        html!("div", {
                            .class("actions-wrapper")
                            .children(&mut [
                                html!("button", {
                                    .class("link-button")
                                    .text("Clone")
                                    .event(clone!(item => move |_event: events::Click| {
                                        crate::utils::log(&item);
                                    }))
                                }),
                                html!("span", {
                                    .text("|")
                                }),
                                html!("button", {
                                    .class("link-button")
                                    .text("Delete")
                                    .event(clone!(item => move |_event: events::Click| {
                                        table_state.remove_item(item.lock_ref().db_id);
                                    }))
                                }),
                            ])
                        })
                    )
                }),
            ])
        })
    }
}
