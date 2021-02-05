use crate::utils::log;
use crate::item::Item;


pub async fn get() -> Vec<Item> {
    let json = r#"
        [
            {
                "db_id": 1,
                "id": "simple",
                "english": "Hello world",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": "",
                "item_kind": "Button",
                "status": "Approved",
                "section": "sec"
            },
            {
                "db_id": 2,
                "id": "complex",
                "english": "{$userName} {$photoCount ->\n    [one] added a new photo\n   *[other] added {$photoCount} new photos\n} to {$userGender ->\n    [male] his stream\n    [female] her stream\n   *[other] their stream\n}.\n",
                "in_app": false,
                "in_element": false,
                "in_mock": false,
                "comments": "fdsa",
                "zeplin_reference": "",
                "item_kind": "Subheading",
                "status": "Approved",
                "section": "sec"
            }
        ]
    "#;
    // let json: String = fs::read_to_string("src/moc-db.json").unwrap();
    let vec: Vec<Item> = serde_json::from_str(&json).unwrap();
    vec
}

pub async fn get_entries() -> Vec<String> {
    vec![
        "JIG".to_string(),
        "Memory game".to_string(),
        "Publish".to_string(),
        "Poster".to_string()
    ]
}

pub fn save(vec: Vec<Item>) {
    log(&vec);
}
