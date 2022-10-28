use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;

pub struct Config {
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Word {
    #[serde(alias = "return")]
    _return: String,
    word: String,
    #[serde(alias = "derivedLang")]
    derived_lang: String,
    explanation: String,
    root: String,
    #[serde(alias = "firstInHistory")]
    first_in_history: String,
    additional: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let args_size = args.len();

        if args_size < 2 {
            return Err("Please enter a query word.");
        } else if args_size > 2 {
            return Err("Please enter one word to query.");
        }

        let query = args[1].trim().to_lowercase().clone();

        if query.chars().count() > 30 {
            return Err("The word length is too big.");
        }

        Ok(Config { query })
    }
}

pub fn request(word: &String) -> Result<(), Box<dyn Error>> {
    let request_url: String = format!("https://api.etimolojiturkce.com/word/{word}", word = &word);

    let byte_resp = reqwest::blocking::get(&request_url)?.bytes()?;
    // let json_resp: Option<Word> = Some(serde_json::from_slice(&byte_resp)?);
    let json_resp: Word = match serde_json::from_slice(&byte_resp) {
        Ok(json_resp) => json_resp,
        Err(_) => Word {
            _return: "".to_string(),
            word: "".to_string(),
            derived_lang: "".to_string(),
            explanation: "".to_string(),
            root: "".to_string(),
            first_in_history: "".to_string(),
            additional: "".to_string(),
        },
    };

    if json_resp._return == "" {
        println!("The word is not found :(");
    } else {
        println!(
            "
            Word: {}, 
            Root: {}, 
            Derived Language: {}, 
            Explanation: {},
            First in History: {},
            Additional Info: {}
            ",
            json_resp.word,
            json_resp.root,
            json_resp.derived_lang,
            json_resp.explanation,
            json_resp.first_in_history,
            json_resp.additional
        );
    }

    Ok(())
}
