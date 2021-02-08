use crate::state::State;
use std::rc::Rc;
use futures_signals::signal_vec::SignalVecExt;
use dominator::{Dom, html, clone};
use super::item::ItemComponent;
use super::select_columns::SelectColumns;


#[derive(Debug)]
pub struct TableComponent {

}

impl TableComponent {
    pub fn render(state: Rc<State>) -> Dom {
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
                    .property("id", "sections")
                    .children_signal_vec(state.sections.signal_vec_cloned()
                        .map(|section| {
                            html!("option", {
                                .property("value", section)
                            })
                        }))
                }),

                html!("datalist", {
                    .property("id", "item-kinds")
                    .children_signal_vec(state.item_kinds.signal_vec_cloned()
                        .map(|item_kind| {
                            html!("option", {
                                .property("value", item_kind)
                            })
                        }))
                }),

                SelectColumns::render(state.clone()),
            ])
        })
    }
}
