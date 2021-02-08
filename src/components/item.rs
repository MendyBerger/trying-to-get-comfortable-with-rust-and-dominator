use url::Url;
use crate::state::Section;
use crate::state::{ItemStatus, ItemKind, Item, State};
use web_sys::HtmlSelectElement;
use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events, with_node};
use std::str::FromStr;
use strum::IntoEnumIterator;


#[derive(Clone)]
pub struct ItemComponent {

}

impl ItemComponent {
    pub fn render(item: Rc<Mutable<Item>>, state: Rc<State>) -> Dom {
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
                        .apply_if(item_ref.section.is_some(), |dom| {
                            dom.property("value", &item_ref.section.clone().unwrap())
                        })
                        .attribute("list", "sections")
                        .event(clone!(item => move |event: events::Input| {
                            let value: Section = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.section = Some(value);
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("input", {
                        .apply_if(item_ref.item_kind.is_some(), |dom| {
                            dom.property("value", &item_ref.item_kind.clone().unwrap())
                        })
                        .attribute("list", "item-kinds")
                        .event(clone!(item => move |event: events::Input| {
                            let value: ItemKind = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.item_kind = Some(value);
                        }))
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
                        .property("type", "url")
                        .apply_if(item_ref.zeplin_reference.is_some(), |dom| {
                            dom.property("value", &item_ref.zeplin_reference.clone().unwrap().to_string())
                        })
                        .event(clone!(item => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let value = Url::parse(&value);

                            if value.is_ok() {
                                let mut item = item.lock_mut();
                                item.zeplin_reference = Some(value.unwrap());
                            }
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
                                        state.remove_item(&item.lock_ref().id);
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
