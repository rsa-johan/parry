use std::env;

pub fn get_next_arg(key: &str) -> Option<String> {
    let args = env::args();
    let mut found = true;
    let mut param = String::new();

    for arg in args {
        if found && arg.starts_with("--") {
            return if param.len() > 0 {
                Some(param.trim().to_string())
            } else {
                None
            };
        } else if found {
            param.push_str(&arg);
            param.push_str(" ");
        }

        if arg == key {
            found = true;
        }
    }
    return if param.len() > 0 {
        Some(param.trim().to_string())
    } else {
        None
    };
}
