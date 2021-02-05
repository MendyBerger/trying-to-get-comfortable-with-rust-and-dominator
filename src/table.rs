use std::rc::Rc;
use wasm_bindgen::prelude::*;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{SignalVecExt, MutableVec};
use dominator::{Dom, html, clone};
use crate::item::{Item, ItemComponent, ItemKind, ItemStatus};


#[derive(Debug)]
pub struct TableComponent {
    items: MutableVec<Rc<Mutable<Item>>>,
    sections: MutableVec<String>
}

impl TableComponent {
    pub fn new(items: &Vec<Item>) -> Rc<Self> {
        let sections = TableComponent::generate_sections(&items);

        // cloning of every item should probably not be happening
        let items = items.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let items = MutableVec::new_with_values(items);
        Rc::new(Self {
            items,
            sections,
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

    fn generate_sections(item_vec: &Vec<Item>) -> MutableVec<String> {
        let section_vec: MutableVec<String> = MutableVec::new();
        for elem in item_vec.iter() {
            section_vec.lock_mut().push_cloned(elem.section.clone());
        };
        section_vec
    }

    pub fn render(state: Rc<Self>) -> Dom {
        // just a placeholder because I don't know how to return 2 children
        html!("div", {
            // inline style because I'd really like to remove this element altogether
            .style("display", "contents")
            .children(&mut [
                html!("table", {
                    .class("ftl-table")
                    .child(
                        html!("tr", {
                            .children(&mut [
                                html!("th", {
                                    .text("ID")
                                }),
                                html!("th", {
                                    .text("Section")
                                }),
                                html!("th", {
                                    .text("Item Kind")
                                }),
                                html!("th", {
                                    .text("English")
                                }),
                                html!("th", {
                                    .text("Status")
                                }),
                                html!("th", {
                                    .text("Zeplin reference")
                                }),
                                html!("th", {
                                    .text("Comments")
                                }),
                                html!("th", {
                                    .class("office-use-only")
                                    .text("App")
                                }),
                                html!("th", {
                                    .class("office-use-only")
                                    .text("Element")
                                }),
                                html!("th", {
                                    .class("office-use-only")
                                    .text("Mock")
                                }),
                                html!("th", {
                                }),
                            ])
                        })
                    )
                    .children_signal_vec(state.items.signal_vec_cloned()
                        .map(clone!(state => move |item| {
                            ItemComponent::render(item.clone(), state.clone())
                        })))
                }),

                html!("datalist", {
                    .property("id", "ice-cream-flavors")
                    .children_signal_vec(state.sections.signal_vec_cloned()
                        .map(|section| {
                            html!("option", {
                                .property("value", section)
                            })
                        }))
                })
            ])
        })
    }
}
