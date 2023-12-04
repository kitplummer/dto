use dittolive_ditto::ditto::Ditto;
use futures::executor::block_on;
use serde_json::Value;

pub fn query(query: String, ditto: &Ditto) {
    match block_on(ditto.store().execute(query, None)) {
        Ok(response) => {
            let items: Vec<Value> = response
                .into_iter()
                .map(|query_result| serde_json::from_str(query_result.json().as_str()).unwrap())
                .collect();

            println!("{}", serde_json::json!(items));
        }
        Err(e) => {
            println!("invalid or unsupported query: {}", e)
        }
    }
}
