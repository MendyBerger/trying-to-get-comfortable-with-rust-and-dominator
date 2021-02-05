use crate::table::TableComponent;
use std::rc::Rc;
use dominator::{Dom, html};
use crate::item::Item;


#[derive(Debug)]
pub struct App {
    items: Vec<Item>,
}

impl App {
    pub async fn new() -> Rc<Self> {
        let items = crate::db_interface::get().await;
        Rc::new(Self {
            items,
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("main", {
            .text("main")
            .children(&mut [
                TableComponent::render(TableComponent::new(&app.items)),
            ])
        })
    }
}
