use std::fs::File;
use std::io::Read;

type KeywordsList = Vec<String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Dialect {
    pub and: KeywordsList,
    pub background: KeywordsList,
    pub but: KeywordsList,
    pub examples: KeywordsList,
    pub feature: KeywordsList,
    pub given: KeywordsList,
    pub name: String,
    pub native: String,
    pub rule: KeywordsList,
    pub scenario: KeywordsList,
    pub scenarioOutline: KeywordsList,
    pub then: KeywordsList,
    pub when: KeywordsList,
}

impl Dialect {
    pub fn dialect_of(locale: &str) -> Dialect {
        let config_file_name = format!("resources/dialects/{}.json", locale);
        let mut dialect_config_file = File::open(config_file_name).unwrap();
        let mut config_contents = String::new();
        dialect_config_file.read_to_string(&mut config_contents).unwrap();
        let dialect: Dialect = serde_json::from_str(&config_contents).unwrap();
        dialect
    }
}