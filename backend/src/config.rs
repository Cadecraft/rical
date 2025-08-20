use std::collections::HashMap;
use std::env;

pub fn get_config() -> HashMap<String, String> {
    let required: Vec<&str> = vec!["DATABASE_URL", "JWT_SECRET"];
    let mut res: HashMap<String, String> = HashMap::new();
    for key in required {
        let val = env::var(key).expect(&format!("{} must be set", key));
        res.insert(key.to_string(), val);
    }
    res
}
