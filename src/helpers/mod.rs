use std::collections::HashMap;
use time;

// Helper functions

pub fn url_encode_hashmap(hashmap: &HashMap<&str, &str>) -> String {
    if hashmap.is_empty() {
        return "".to_string();
    }
    let mut acc = "".to_string();
    for (name, param) in hashmap {
        acc += &(name.to_string() + "=" + param + "&");
    }
    acc.pop(); // remove the last "&"
    acc
}

pub fn get_unix_timestamp_ms() -> i64 {
    let current_time = time::get_time();
    //Calculate milliseconds
    (current_time.sec as i64 * 1000) + (current_time.nsec as i64 / 1000 / 1000)
}

pub fn strip_empties(x: &mut HashMap<&str, &str>) {
    let empties: Vec<_> = x.iter()
        .filter(|&(_, &v)| v.is_empty())
        .map(|(k, _)| (*k).clone())
        .collect();
    for empty in empties {
        x.remove(&empty);
    }
}
