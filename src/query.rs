use dittolive_ditto::ditto::Ditto;
use futures::executor::block_on;
use serde_json::Value;

pub fn query(query: String, ditto: &Ditto) {
    let response = block_on(ditto.store().execute(query, None)).unwrap();

    let items: Vec<Value> = response
        .into_iter()
        .map(|query_result| serde_json::from_str(query_result.json().as_str()).unwrap())
        .collect();

    println!("{}", serde_json::json!(items));
}
