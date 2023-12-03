use dittolive_ditto::{ditto::Ditto, store::dql::QueryResult};

pub fn observe(query: String, ditto: &Ditto) {
    match ditto.sync().register_subscription(query.to_owned(), None) {
        Ok(_response) => {
            let _change_observer = ditto
                .store()
                .register_observer(query, None, move |result: QueryResult| {
                    if result.item_count() > 0 {
                        println!("result count: {}", result.item_count());
                        for item in result {
                            println!("{}", item.json());
                        }
                    }
                })
                .unwrap();

            // CTRL-C to exit
            loop {}
        }
        Err(e) => {
            println!("invalid or unsupported query: {}", e)
        }
    }
    //println!("subscribed to {}", sub);
}
