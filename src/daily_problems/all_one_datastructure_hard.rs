use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct AllOne {
    freq_map: HashMap<String, usize>,
    count_map: BTreeMap<usize, Vec<String>>,
}

trait NewTrait {
    fn new() -> Self;

    fn inc(&mut self, key: String);

    fn dec(&mut self, key: String);

    fn get_max_key(&self) -> String;

    fn get_min_key(&self) -> String;
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NewTrait for AllOne {
    fn new() -> Self {
        Self {
            freq_map: HashMap::new(),
            count_map: BTreeMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let count = self.freq_map.entry(key.clone()).or_insert(0);
        if *count > 0 {
            self.count_map.get_mut(count).unwrap().retain(|x| x != &key);
            if self.count_map[count].is_empty() {
                self.count_map.remove(count);
            }
        }
        *count += 1;
        self.count_map
            .entry(*count)
            .or_insert_with(Vec::new)
            .push(key);
    }

    fn dec(&mut self, key: String) {
        if let Some(count) = self.freq_map.get_mut(&key) {
            self.count_map.get_mut(count).unwrap().retain(|x| x != &key);
            if self.count_map[count].is_empty() {
                self.count_map.remove(count);
            }
            *count -= 1;
            if *count == 0 {
                self.freq_map.remove(&key);
            } else {
                self.count_map
                    .entry(*count)
                    .or_insert_with(Vec::new)
                    .push(key);
            }
        }
    }

    fn get_max_key(&self) -> String {
        self.count_map
            .iter()
            .next_back()
            .and_then(|(_, v)| v.first())
            .cloned()
            .unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.count_map
            .iter()
            .next()
            .and_then(|(_, v)| v.first())
            .cloned()
            .unwrap_or_default()
    }
}

#[test]
fn itwortks() {
    let key = String::from("narendra");
    let mut obj = AllOne::new();
    obj.inc(key.clone());
    println!("{:?}", obj);
    obj.dec(key);
    println!("{:?}", obj);
    let ret_3: String = obj.get_max_key();
    println!("ret3 {ret_3}");
    let ret_4: String = obj.get_min_key();
    println!("ret4 {ret_4}");
}
