use dittolive_ditto::ditto::Ditto;

pub fn subscribe(query: String, ditto: &Ditto) {
    let sub = ditto.sync().register_subscription(query, None).unwrap();
    println!("subscribed - {}", sub);
}
