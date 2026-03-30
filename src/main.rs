
use std::collections::HashMap;

fn count_word_occurrence(text: &str) -> HashMap<&str, usize> {

    let mut map: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {

        let count = map.entry(word).or_insert(0);

        // count is a reference to the value in the map, so we dereference it to update the count
        *count += 1;
    }

    map
}

fn main() -> () {

    let mut hash_map = HashMap::new();

    hash_map.insert(String::from("key1"), String::from("value1"));
    hash_map.insert(String::from("key2"), String::from("value2"));
    hash_map.insert(String::from("key3"), String::from("value3"));
    hash_map.insert(String::from("key4"), String::from("value4"));
    hash_map.insert(String::from("key5"), String::from("value5"));
    hash_map.insert(String::from("key6"), String::from("value6"));
    println!("{:#?}", hash_map);

    // accessing values
    let value1 = hash_map.get("key1");
    let value1 = match value1 {
        Some(value) => value,
        None => "Key not found",
    };
    println!("Value for key1: {:?}", value1);

    for (key, value) in &hash_map {
        println!("{}: {}", key, value);
    }

    // to check if a key exists, if not we can use to_insert on entry to add
    // that key with a value.
    hash_map.entry(String::from("ke14")).or_insert(String::from("default_value"));
    println!("{:#?}", hash_map);

    let map = count_word_occurrence("This is my awsome and words list where I list awsome!");
    println!("{:#?}", map);

}
