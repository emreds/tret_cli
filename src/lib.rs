use std::error::Error;

pub struct Config {
    pub query: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str > {
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

        Ok(Config{query})
    }
    
}

pub fn request(word: &String) -> Result<(), Box<dyn Error>> {
    
    let request_url = format!(
        "https://www.etimolojiturkce.com/kelime/{word}",
        word = &word
    );

    let resp = reqwest::blocking::get(&request_url)?.text()?;
    println!("{:#?}", resp);

    Ok(())
}