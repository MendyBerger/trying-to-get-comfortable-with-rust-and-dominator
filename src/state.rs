use web_sys::HtmlDialogElement;
use std::rc::Rc;
use std::clone::Clone;
use serde_derive::{Deserialize, Serialize};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use strum_macros::{EnumString, Display, EnumIter};

pub struct State {
    pub entries: Vec<String>,
    pub selected_entry: String,
    pub items: MutableVec<Rc<Mutable<Item>>>,
    pub sections: MutableVec<String>,
    pub visible_columns: MutableVec<String>,
    pub hidden_columns: MutableVec<String>,
    pub dialog_ref: Mutable<Option<HtmlDialogElement>>,
}

impl State {
    pub async fn new() -> State {
        let items = crate::db_interface::get().await;
        let sections = Self::generate_sections(&items);
        let items = items.iter().map(|i| Rc::new(Mutable::new(i.clone()))).collect();
        let items = MutableVec::new_with_values(items);
        let entries = crate::db_interface::get_entries().await;
        let selected_entry = entries[0].clone();


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
            selected_entry,
            items,
            sections,
            visible_columns,
            hidden_columns,
            dialog_ref: Mutable::new(None),
        }
    }

    pub fn add_item(&self) {
        let mut vec = self.items.lock_mut();
        let itr = vec.iter();
        let last = itr.last();
        let last = last.unwrap();
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
}


#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq)]
pub enum ItemKind {
    Heading,
    Subheading,
    Button,
    Instruction,
    Toggle,
    Warning,
    Feedback,
    #[strum(serialize = "Help Text")]
    HelpText,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, EnumIter, PartialEq)]
pub enum ItemStatus {
    Approved,
    Discuss,
    #[strum(serialize = "On Hold")]
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
