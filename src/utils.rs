use dittolive_ditto::store::Store;
use std::collections::HashMap;

pub fn show_storage_resources_used(store: &Store) {
    let resources = store.disk_usage().exec();
    let mut hm = HashMap::new();
    hm.insert("path".to_string(), resources.path.to_string());
    hm.insert(
        "size_in_bytes".to_string(),
        resources.size_in_bytes.to_string(),
    );
    println!("{}", serde_json::json!(hm));
}
