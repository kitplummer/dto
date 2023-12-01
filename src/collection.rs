use dittolive_ditto::store::Store;

pub fn list_collections(store: &Store) {
    let collections = store.collection_names().unwrap();
    println!("{}", serde_json::json!(collections));
}
