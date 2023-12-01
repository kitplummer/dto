use dittolive_ditto::ditto::Ditto;

pub fn presence(query: String, ditto: &Ditto) {
    let presence = ditto.presence().exec();
    match query.as_str() {
        "remote peers" => {
            println!("remote peers: {:#?}", presence.remote_peers);
        }
        "local peer" => {
            println!("local peer: {:#?}", presence.local_peer);
        }
        "device name" => {
            println!("device name: {:#?}", presence.local_peer.device_name);
        }
        &_ => {
            println!("not a valid command.");
        }
    }
}
