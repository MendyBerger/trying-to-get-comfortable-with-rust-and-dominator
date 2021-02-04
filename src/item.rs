use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events};
use crate::App;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub id: i32,
    pub key: String,
    pub value: String,
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
                        .property("value", &item_ref.key)
                        .event(clone!(item, app => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.key = value;
                        }))
                    }))
                }),
                html!("td", {
                    .child(html!("textarea", {
                        .text(&item_ref.value)
                        .event(clone!(item, app => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut item = item.lock_mut();
                            item.key = value;
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
                        app.remove_item(item.lock_ref().id);
                    }))
                }),
            ])
        })
    }
}
