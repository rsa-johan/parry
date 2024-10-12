use std::env;

pub fn get_next_arg(key: &str) -> Option<String> {
    let mut args = env::args();
    while let Some(v) = args.next() {
        if v == key {
            return args.next();
        }
    }
    None
}
