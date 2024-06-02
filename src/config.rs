use ini::ini;

pub struct Configuration {
    pub repository:String,
    pub min_disabled_words:usize,
    pub max_disabled_words:usize
}

pub(crate) fn load_config() -> Configuration {
    let mut config: Configuration = Configuration {
        repository: "config.ini".to_string(),
        min_disabled_words: 0,
        max_disabled_words: 15,
    };
    let file = ini!("config.ini");
    match file["randomizer"]["repository"].clone() {
        None => {
            println!("Path to repository not found in config.ini, taking default value words.txt");
        },
        Some(val) => config.repository = val
    }
    match file["randomizer"]["min_disabled_words"].clone() {
        None => {
            println!("Minimum disabled words number not found in config.ini, taking default value 0");
        },
        Some(val) => {
            match val.parse::<usize>() {
                Ok(val) => {
                    config.min_disabled_words = val;
                }
                Err(_) => {
                    println!("Minimum disabled words is incorrect in config.ini, taking default value 0");
                }
            }
        }
    }
    match file["randomizer"]["max_disabled_words"].clone() {
        None => {
            println!("Maximum disabled words number not found in config.ini, taking default value 15");
        },
        Some(val) => {
            match val.parse::<usize>() {
                Ok(val) => {
                    config.min_disabled_words = val;
                }
                Err(_) => {
                    println!("Maximum disabled words is incorrect in config.ini, taking default value 15");
                }
            }
        }
    }
    return config;
}