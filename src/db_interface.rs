use crate::utils::get_random_string;
use crate::utils::log;
use crate::state::{Translation, TranslationStatus};


pub async fn get_entries() -> Vec<String> {
    vec![
        "JIG".to_string(),
        "Memory game".to_string(),
        "Publish".to_string(),
        "Poster".to_string(),
    ]
}

pub async fn get_translations(entries: &Vec<&String>) -> Vec<Translation> {
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
    let vec: Vec<Translation> = serde_json::from_str(&json).unwrap();
    vec
}

pub async fn clone_translation(translation: &Translation) -> Translation {
    let mut translation = translation.clone();
    translation.id = get_random_string(10);
    log(&translation);
    translation
}

pub async fn create_translation() -> Translation {
    Translation {
        id: get_random_string(10),
        english: String::new(),
        hebrew: String::new(),
        section: None,
        item_kind: None,
        status: TranslationStatus::Discuss,
        zeplin_reference: None,
        comments: String::new(),
        in_app: false,
        in_element: false,
        in_mock: false,
    }
}

pub async fn save_translation(translation:Translation) ->Translation {
    log(&translation);
    translation
}
