mod utils;

use std::collections::HashMap;
use utils::runtime::get_next_arg;

pub struct Cmd {
    params: Vec<String>,
    map: HashMap<String, String>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            params: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn arg(&mut self, name: String, alias: Option<String>) -> &mut Self {
        let mut key = name;
        if let Some(alias_name) = alias {
            key.push_str(":");
            key.push_str(&alias_name);
        }
        self.params.push(key);
        return self;
    }

    pub fn parse(&mut self) -> &mut Self {
        for param in self.params.iter() {
            let mut m = "--".to_string();
            let b: Vec<&str> = param.splitn(2, ":").collect();
            param.split(":").for_each(|x| {
                m.push_str(x);
                let key = b.get(0).unwrap().to_string();
                if let Some(val) = get_next_arg(&m) {
                    self.map.insert(key, val);
                }
                m = "-".to_string();
            });
        }

        return self;
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}
