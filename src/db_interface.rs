use std::collections::HashMap;
use crate::utils::log;
use crate::state::Item;


pub async fn get() -> Vec<Item> {
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
    // let json: String = fs::read_to_string("src/moc-db.json").unwrap();
    let vec: Vec<Item> = serde_json::from_str(&json).unwrap();
    vec
}

pub async fn get_entries() -> HashMap<String, bool> {
    let mut map = HashMap::new();
    map.insert("JIG".to_string(), false);
    map.insert("Memory game".to_string(), false);
    map.insert("Publish".to_string(), true);
    map.insert("Poster".to_string(), false);
    map
}

pub fn save(vec: Vec<Item>) {
    log(&vec);
}
