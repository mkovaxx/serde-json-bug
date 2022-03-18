const JSON_TEXT: &str = r#"
{
    "animals": {
        "items": [
            {"animal_tag": "cat"},
            {"animal_tag": "xenomorph"},
            {"animal_tag": "dog"}
        ]
    }
}
"#;

fn main() {
    let stuff = serde_json::from_str::<Things>(JSON_TEXT);
    println!("{:?}", stuff);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum Things {
    Animals(Animals),
    Plants(Plants),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Animals {
    items: Vec<Animal>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "animal_tag", rename_all = "snake_case")]
enum Animal {
    Cat,
    Dog,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Plants {}
