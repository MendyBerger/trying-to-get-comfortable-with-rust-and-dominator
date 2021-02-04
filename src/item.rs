use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events};
use crate::App;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemKind {
    Heading,
    Subheading,
    Button,
    Instruction,
    Toggle,
    Warning,
    Feedback,
    HelpText,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemStatus {
    Approved,
    Discuss,
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
    pub fn render(item: Rc<Mutable<Item>>, app: Rc<App>) -> Dom {
        let item_ref = item.lock_ref();
        html!("tr", {
            .children(&mut [
                html!("td", {
                    .child(html!("input", {
                        .property("value", &item_ref.id)
                        .event(clone!(item, app => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.id = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("textarea", {
                        .text(&item_ref.english)
                        .event(clone!(item, app => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.english = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("button", {.text("log")}))
                    .event(clone!(item => move |_event: events::Click| {
                        crate::utils::log(&item);
                    }))
                }),
                html!("td", {
                    .child(html!("button", {.text("-")}))
                    .event(clone!(item, app => move |_event: events::Click| {
                        app.remove_item(item.lock_ref().db_id);
                    }))
                }),
            ])
        })
    }
}
