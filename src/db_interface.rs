use crate::utils::log;
use crate::item::Item;


pub async fn get() -> Vec<Item> {
    let json = r#"
        [
            {
                "id": 1,
                "key": "simple",
                "value": "Hello world"
            },
            {
                "id": 2,
                "key": "complex",
                "value": "{$userName} {$photoCount ->\n            [one] added a new photo\n           *[other] added {$photoCount} new photos\n        } to {$userGender ->\n            [male] his stream\n            [female] her stream\n           *[other] their stream\n        }."
            }
        ]
    "#;
    // let json: String = fs::read_to_string("src/moc-db.json").unwrap();
    let vec: Vec<Item> = serde_json::from_str(&json).unwrap();
    vec
}

pub fn save(vec: Vec<Item>) {
    log(&vec);
}
