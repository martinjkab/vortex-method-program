use std::collections::HashMap;

pub trait Parametrized {
    fn parametrize(&self, params: HashMap<&str, &str>) -> String;
}

impl Parametrized for String {
    fn parametrize(&self, params: HashMap<&str, &str>) -> String {
        let mut copy = self.to_string();
        for (key, value) in params.iter() {
            let p_string = format!("${}", key);
            copy = copy.replace(&p_string, &value);
        }
        copy
    }
}
