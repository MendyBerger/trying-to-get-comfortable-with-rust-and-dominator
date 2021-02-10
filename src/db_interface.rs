use crate::utils::get_random_string;
use crate::utils::log;
use crate::state::{Item, ItemStatus};


pub async fn get_entries() -> Vec<String> {
    vec![
        "JIG".to_string(),
        "Memory game".to_string(),
        "Publish".to_string(),
        "Poster".to_string(),
    ]
}

pub async fn get_translations(entries: &Vec<&String>) -> Vec<Item> {
    println!("{:?}", entries);
    let json = r#"
        [
            {
                "id": "simple",
                "english": "Hello world",
                "hebrew": "כגד",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": null,
                "item_kind": "Button",
                "status": "Approved",
                "section": "sec1"
            },
            {
                "id": "complex",
                "english": "{$userName} {$photoCount ->\n    [one] added a new photo\n   *[other] added {$photoCount} new photos\n} to {$userGender ->\n    [male] his stream\n    [female] her stream\n   *[other] their stream\n}.\n",
                "hebrew": "כגלםממך",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": "https://google.com",
                "item_kind": "Subheading",
                "status": "Approved",
                "section": "sec2"
            }
        ]
    "#;
    let vec: Vec<Item> = serde_json::from_str(&json).unwrap();
    vec
}

pub async fn clone_translation(item: &Item) -> Item {
    let mut item = item.clone();
    item.id = get_random_string(10);
    log(&item);
    item
}

pub async fn create_translation() -> Item {
    Item {
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
    }
}

pub async fn save_translation(item: Item) -> Item {
    log(&item);
    item
}
