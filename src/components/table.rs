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
                html!("div", {
                    .class("ftl-table")
                    .child(
                        html!("div", {
                            .class("ftl-row")
                            .children(&mut [
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("ID")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Section")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Item Kind")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("English")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Status")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Zeplin reference")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Comments")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("App")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("Element")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("Mock")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
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
