use crate::state::State;
use crate::components::table::TableComponent;
use std::rc::Rc;
use dominator::{Dom, html, events, with_node, clone};
use web_sys::HtmlSelectElement;


#[derive(Debug)]
pub struct App {
}

impl App {
    pub async fn render() -> Dom {
        let state: Rc<State> = Rc::new(State::new().await);

        html!("main", {
            .children(&mut [
                html!("select" => HtmlSelectElement, {
                    .attribute("multiple", "")
                    .with_node!(elem => {
                        .event(clone!(elem => move |_:events::Change| {
                            let selected_entry: String = elem.value();
                            crate::utils::log(&selected_entry);
                        }))
                    })
                    .children(
                        state.entries.iter().map(|(e, selected)| {
                            html!("option", {
                                .property("text", e.to_string())
                                .property("value", e.to_string())
                                .property("selected", selected.clone())
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
                            .event(clone!(state => move |_event: events::Click| {
                                state.dialog_ref
                                    .lock_ref().clone().expect("Can't get dialog")
                                    .show_modal().expect("Can't open dialog");
                            }))
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
                            .event(clone!(state => move |_event: events::Click| {
                                state.add_item();
                            }))
                        }),
                        html!("span", {
                            .text("Add a text")
                        }),
                    ])
                }),
                TableComponent::render(state),
            ])
        })
    }
}
