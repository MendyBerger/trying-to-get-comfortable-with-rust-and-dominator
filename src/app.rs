use std::slice::Iter;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{SignalVecExt, MutableVec, MutableVecLockRef};
use dominator::{Dom, html, clone, events};
use crate::db_interface::save;
use crate::item::{Item, ItemComponent, ItemKind, ItemStatus};


#[derive(Debug)]
pub struct App {
    items: MutableVec<Rc<Mutable<Item>>>
}

impl App {
    pub async fn new() -> Rc<Self> {
        let items = crate::db_interface::get().await;
        // cloning of every item should probably not be happening
        let items = items.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let items = MutableVec::new_with_values(items);
        Rc::new(Self {
            items
        })
    }


    pub fn add_item(&self) {
        let mut vec = self.items.lock_mut();
        let itr = vec.iter();
        let last = itr.last();
        let last = last.unwrap_throw();
        let next_id = last.lock_ref().db_id + 1;
        vec.push_cloned(Rc::new(Mutable::new(Item {
            // No! Please don't do this!
            db_id: next_id + 1,
            id: String::new(),
            english: String::new(),

            section: String::new(),
            item_kind: ItemKind::Heading,
            status: ItemStatus::Discuss,
            zeplin_reference: String::new(),
            comments: String::new(),
            in_app: false,
            in_element: false,
            in_mock: false,
        })));
    }

    pub fn remove_item(&self, id: i32) {
        let mut vec = self.items.lock_mut();
        let index = vec.iter().position(|i| i.lock_ref().db_id == id).unwrap();
        vec.remove(index);
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("main", {
            .text("main")
            .children(&mut [
                html!("table", {
                    .children_signal_vec(app.items.signal_vec_cloned()
                        .map(clone!(app => move |item| {
                            ItemComponent::render(item.clone(), app.clone())
                        })))
                }),

                html!("button", {
                    .text("+")
                    .event(clone!(app => move |_event: events::Click| {
                        app.add_item();
                        
                    }))
                }),

                html!("button", {
                    .text("Save")
                    .event(clone!(app => move |_event: events::Click| {
                        let items: MutableVecLockRef<Rc<Mutable<Item>>> = app.items.lock_ref();
                        let items: Iter<'_, Rc<Mutable<Item>>> = items.iter();
                        let items = items.map(|i: &Rc<Mutable<Item>>| i.lock_ref().clone());
                        let items = items.collect::<Vec<Item>>();

                        save(items);
                    }))
                }),
            ])
        })
    }
}
