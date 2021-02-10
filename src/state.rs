use dominator_helpers::futures::AsyncLoader;
use std::collections::HashMap;
use crate::db_interface;
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
    pub loader: Rc<AsyncLoader>,
}

impl State {
    pub async fn new() -> State {
        let entries: HashMap<String, bool> = db_interface::get_entries()
            .await
            .iter()
            .map(|entry| (entry.clone(), true))
            .collect();

        // this should probably react to a signal update
        let visible_entries: Vec<&String> = entries
            .iter()
            .filter(|entry| *entry.1)
            .map(|entry| entry.0)
            .collect();
        let items = db_interface::get_translations(&visible_entries).await;
        let sections = Self::generate_sections(&items);
        let item_kinds = Self::generate_item_kinds(&items);
        let items = items.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let items = MutableVec::new_with_values(items);


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
            loader: Rc::new(AsyncLoader::new()),
        }
    }

    pub async fn add_item(&self) {
        let item = db_interface::create_translation().await;
        let mut vec = self.items.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(item)));
    }

    pub async fn clone_item(&self, item: &Item) {
        let item = db_interface::clone_translation(&item).await;
        let mut vec = self.items.lock_mut();
        vec.push_cloned(Rc::new(Mutable::new(item)));
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
