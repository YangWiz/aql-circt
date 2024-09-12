use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AQLType {
    Base(String),
    Ordering
}

#[derive(Clone)]
pub enum Ordering {
    FIFO(String),
    HASH(String),
    STACK(String)
}

#[derive(Debug, Clone)]
pub struct ConversionTable {
    tbs: HashMap<String, AQLType>,
}

impl ConversionTable {
    pub fn new() -> Self {
        let mut tbs = HashMap::new();

        tbs.insert(String::from("bool"), AQLType::Base(String::from("i1")));
        tbs.insert(String::from("int"), AQLType::Base(String::from("i32")));
        tbs.insert(String::from("i32"), AQLType::Base(String::from("i32")));
        tbs.insert(String::from("i64"), AQLType::Base(String::from("i64")));
        tbs.insert(String::from("element_ordering"), AQLType::Ordering);

        ConversionTable {
            tbs
        }
    }

    pub fn convert(&self, t: &String) -> AQLType {
        let key = t.trim();
        self.tbs[key].clone()
    }
}
