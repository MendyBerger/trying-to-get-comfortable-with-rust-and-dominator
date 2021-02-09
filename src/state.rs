use std::collections::HashMap;
use crate::utils::get_random_string;
use url::Url;
use web_sys::HtmlDialogElement;
use std::rc::Rc;
use std::clone::Clone;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use strum_macros::{EnumString, Display, EnumIter};

pub struct State {
    pub entries: HashMap<String, bool>,
    pub items: MutableVec<Rc<Mutable<Item>>>,
    pub sections: MutableVec<String>,
    pub item_kinds: MutableVec<String>,
    pub visible_columns: MutableVec<String>,
    pub hidden_columns: MutableVec<String>,
    pub dialog_ref: Mutable<Option<HtmlDialogElement>>,
}

impl State {
    pub async fn new() -> State {
        let items = crate::db_interface::get().await;
        let sections = Self::generate_sections(&items);
        let item_kinds = Self::generate_item_kinds(&items);
        let items = items.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let items = MutableVec::new_with_values(items);
        let entries = crate::db_interface::get_entries().await;


        let visible_columns = vec![
            "ID".to_string(),
            "Section".to_string(),
            "Item Kind".to_string(),
            "English".to_string(),
            "Status".to_string(),
            "Zeplin reference".to_string(),
            "Comments".to_string(),
        ];
        let hidden_columns = vec![
            "App".to_string(),
            "Element".to_string(),
            "Mock".to_string(),
        ];
        let visible_columns = MutableVec::new_with_values(visible_columns);
        let hidden_columns = MutableVec::new_with_values(hidden_columns);
        Self {
            entries,
            items,
            sections,
            item_kinds,
            visible_columns,
            hidden_columns,
            dialog_ref: Mutable::new(None),
        }
    }

    pub fn add_item(&self) {
        let mut vec = self.items.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(Item {
            id: get_random_string(10),
            english: String::new(),
            hebrew: String::new(),
            section: None,
            item_kind: None,
            status: ItemStatus::Discuss,
            zeplin_reference: None,
            comments: String::new(),
            in_app: false,
            in_element: false,
            in_mock: false,
        })));
    }

    pub fn clone_item(&self, item: &Item) {
        let mut item = item.clone();
        item.id = get_random_string(10);
        let item = Rc::new(Mutable::new(item));
        let mut vec = self.items.lock_mut();
        vec.push_cloned(item);
    }

    pub fn remove_item(&self, id: &str) {
        let mut vec = self.items.lock_mut();
        let index = vec.iter().position(|i| i.lock_ref().id == id).unwrap();
        vec.remove(index);
    }

    // this and generate_item_kinds should be consolidated somehow into one method
    fn generate_sections(item_vec: &Vec<Item>) -> MutableVec<String> {
        let section_vec: MutableVec<String> = MutableVec::new();
        for elem in item_vec.iter() {
            let section = &elem.section;
            if section.is_some() {
                section_vec.lock_mut().push_cloned(section.clone().unwrap());
            }
        };
        section_vec
    }

    fn generate_item_kinds(item_vec: &Vec<Item>) -> MutableVec<String> {
        let section_vec: MutableVec<String> = MutableVec::new();
        for elem in item_vec.iter() {
            let item_kind = &elem.item_kind;
            if item_kind.is_some() {
                section_vec.lock_mut().push_cloned(item_kind.clone().unwrap());
            }
        };
        section_vec
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq)]
pub enum ItemStatus {
    Approved,
    Discuss,
    #[strum(serialize = "On Hold")]
    OnHold,
}

pub type Section = String;
pub type ItemKind = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub section: Option<Section>,
    pub item_kind: Option<ItemKind>,
    pub english: String,
    pub hebrew: String,
    pub status: ItemStatus,
    pub zeplin_reference: Option<Url>,
    pub comments: String,
    pub in_app: bool,
    pub in_element: bool,
    pub in_mock: bool,
}
