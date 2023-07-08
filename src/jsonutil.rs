use serde::{Deserialize, Serialize};

pub(crate) fn format_json<T: Serialize>(data: T) -> String {
    match serde_json::to_string(&data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error while formatting JSON: {}", e);
            std::process::exit(7);
        }
    }
}

pub(crate) fn parse_json<'a, T: Deserialize<'a>>(input: &'a str) -> T {
    match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Failed to parse string {:?} to structs with error {:?}",
                input, e
            );
            std::process::exit(4)
        }
    }
}
