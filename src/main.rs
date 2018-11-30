extern crate env_logger;
extern crate libp2p;
extern crate futures;
extern crate tokio;

mod engine;

use futures::prelude::*;
use libp2p::{
    Transport,
    core::upgrade::{self, OutboundUpgradeExt},
    secio,
    mplex,
    tokio_codec::{FramedRead, LinesCodec}
};

static TESTIP: &str = "172.18.128.15:35035";

fn main() {
    env_logger::init();

    println!("Hello, world!");

    let local_key = secio::SecioKeyPair::ed25519_generated().unwrap();
    let local_peer_id = local_key.to_peer_id();
    println!("Local peer id: {:?}", local_peer_id);

    // create local DB OR open local DB
    let db = engine::db::MolvaDB::open("molvadbpath".to_string());

    // add test user
    db.put(b"my key", b"my value");

    // get raw test user
    // parse test user and get Friend struct
    // search peer by root nodes from github || root nodes from embedded db
    // open TESTIP

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = libp2p::CommonTransport::new()
        .with_upgrade(secio::SecioConfig::new(local_key))
        .and_then(move |out, _| {
            let peer_id = out.remote_key.into_peer_id();
            let upgrade = mplex::MplexConfig::new().map_outbound(move |muxer| (peer_id, muxer) );
            upgrade::apply_outbound(out.stream, upgrade).map_err(|e| e.into_io_error())
        });

    // Create a Floodsub topic
    let floodsub_topic = libp2p::floodsub::TopicBuilder::new("chat").build();

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut behaviour = libp2p::floodsub::FloodsubBehaviour::new(local_peer_id);
        behaviour.subscribe(floodsub_topic.clone());
        libp2p::Swarm::new(transport, behaviour, libp2p::core::topology::MemoryTopology::empty())
    };

    // Listen on all interfaces and whatever port the OS assigns
    let addr = libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    println!("Listening on {:?}", addr);

    // Reach out to another node
    if let Some(to_dial) = std::env::args().nth(1) {
        let dialing = to_dial.clone();
        // need to sereliaze to dial to Vec<u8>
        match to_dial.parse() {
            Ok(to_dial) => {
                match libp2p::Swarm::dial_addr(&mut swarm, to_dial) {
                    Ok(_) => println!("Dialed {:?}", dialing),
                    Err(e) => println!("Dial {:?} failed: {:?}", dialing, e)
                }
            },
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // Read full lines from stdin
    let stdin = tokio_stdin_stdout::stdin(0);
    let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new());

    // Kick it off
    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => swarm.publish(&floodsub_topic, line.as_bytes()),
                Async::Ready(None) => panic!("Stdin closed"),
                Async::NotReady => break,
            };
        }

        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(message)) => {
                    println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }));

}
