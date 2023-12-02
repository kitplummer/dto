use dittolive_ditto::ditto::Ditto;

pub fn presence(query: String, ditto: &Ditto) {
    let presence = ditto.presence().exec();
    match query.as_str() {
        "remote" => {
            observe_presence(ditto);
        }
        "all" => {
            println!("{}", serde_json::json!(presence));
        }
        "local" => {
            println!("{}", serde_json::json!(presence.local_peer));
        }
        "device name" => {
            println!("device name: {:#?}", presence.local_peer.device_name);
        }
        &_ => {
            println!("not a valid command.");
        }
    }
}

// Run handler for presence changes, loops forever - CTRL-C to exit
pub fn observe_presence(ditto: &Ditto) {
    let _observer = ditto.presence().observe(|graph| {
        println!(
            "mesh change observed (connected to BP? {}.)",
            &graph.local_peer.is_connected_to_ditto_cloud
        );

        println!("remote peers:");
        for peer in &graph.remote_peers {
            let connection_type = peer.connections[0].connection_type;
            println!(
                "  peer: {}, type: {:?}, sdk:{}",
                peer.device_name,
                connection_type,
                peer.ditto_sdk_version.as_ref().unwrap(),
            );
        }
    });

    loop {}
}
