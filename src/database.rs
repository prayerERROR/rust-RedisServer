use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Data {
    value: String,
    expire_time: Option<u128>, 
}

pub struct DataBase {
    cache: HashMap<String, Data>,
}

impl DataBase {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn insert(&mut self, key: String, value: String, valid_time: Option<u64>) {
        // Get expire time
        let expire_time = match valid_time {
            Some(duration) => Some(
                SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap().as_millis() + Duration::from_millis(duration).as_millis()
            ),
            None => None,
        };

        // Insert to redis cache
        let data = Data {
            value: value,
            expire_time: expire_time,
        };
        self.cache.insert(key, data);
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap().as_millis();

        match self.cache.get(key) {
            Some(data) => {
                match data.expire_time {
                    Some(time) => if time > current_time {
                        Some(data.value.clone())
                    } else {
                        self.cache.remove(key);
                        None
                    },
                    None => Some(data.value.clone()),
                }
            },
            None => None,
        }
    }
}