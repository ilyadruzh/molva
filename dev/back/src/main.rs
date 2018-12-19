extern crate env_logger;
#[macro_use]
extern crate libp2p;
extern crate futures;
extern crate tokio;

// mod engine;

use futures::prelude::*;
use libp2p::{
    Transport, NetworkBehaviour,
    core::upgrade::{self, OutboundUpgradeExt},
    secio,
    core::swarm,
    mplex,
    tokio_codec::{FramedRead, LinesCodec}
};
use std::env;
use std::io::{self, Write};

static TESTIP: &str = "172.18.128.15:35035";

fn main() {
    env_logger::init();

    println!("Hello! This is p2p messenger");

    let nickname = {
        print!("Please enter a nickname: ");
        io::stdout().flush().unwrap();
        let mut nickname = String::new();
        let _ = io::stdin().read_line(&mut nickname);
        let nickname = nickname.trim().to_string();
        if !nickname.is_empty() {
            nickname
        } else {
            let name = names::Generator::default().next().unwrap();
            println!("Automatically generated nickname: {}", name);
            name
        }
    };

    let local_key = secio::SecioKeyPair::ed25519_generated().unwrap();
    let local_peer_id = local_key.to_peer_id();
    println!("local_peer id: {:?}", local_peer_id);
    let local_pub_key = local_key.to_public_key();
    println!("local_pub__key: {:?}", local_pub_key);

    // create local DB OR open local DB..
    // let db = engine::db::MolvaDB::open("molvadbpath".to_string());

    // add test user
    // db.put(b"my key", b"my value");

    // get raw test user
    // parse test user and get Friend struct
    // search peer by root nodes from github || root nodes from embedded db
    // open TESTIP

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
 //   let transport =  libp2p::CommonTransport::new()
   //     .with_upgrade(secio::SecioConfig::new(local_key))
     //   .and_then(move |out, _| {
       //     let peer_id = out.remote_key.into_peer_id();
         //   let upgrade = mplex::MplexConfig::new().map_outbound(move |muxer| (peer_id, muxer) );
           // upgrade::apply_outbound(out.stream, upgrade).map_err(|e| e.into_io_error())
       // });

    let transport = libp2p::build_development_transport(local_key);

    // Create a Floodsub topic
    let floodsub_topic = libp2p::floodsub::TopicBuilder::new("chat").build();

    // We create a custom network behaviour that combines floodsub and mDNS.
    // In the future, we want to improve libp2p to make this easier to do.
    #[derive(NetworkBehaviour)]
    struct MyBehaviour<TSubstream: libp2p::tokio_io::AsyncRead + libp2p::tokio_io::AsyncWrite> {
        #[behaviour(handler = "on_floodsub")]
        floodsub: libp2p::floodsub::Floodsub<TSubstream>,
        mdns: libp2p::mdns::Mdns<TSubstream>,
    }

    impl<TSubstream: libp2p::tokio_io::AsyncRead + libp2p::tokio_io::AsyncWrite> MyBehaviour<TSubstream> {
        // Called when `floodsub` produces an event.
        fn on_floodsub<TTopology>(&mut self, message: <libp2p::floodsub::Floodsub<TSubstream> as libp2p::core::swarm::NetworkBehaviour<TTopology>>::OutEvent)
            where TSubstream: libp2p::tokio_io::AsyncRead + libp2p::tokio_io::AsyncWrite
        {
            println!("{}", String::from_utf8_lossy(&message.data));
        }
    }

    // Create a Swarm to manage peers and events
    let mut swarm = {

        let mut behaviour = MyBehaviour {
            floodsub: libp2p::floodsub::Floodsub::new(local_pub_key.clone().into_peer_id()),
            mdns: libp2p::mdns::Mdns::new().expect("Failed to create mDNS service"),
        };

//      let mut behaviour = libp2p::floodsub::FloodsubBehaviour::new(local_peer_id);
//      behaviour.subscribe(floodsub_topic.clone());
        behaviour.floodsub.subscribe(floodsub_topic.clone());

        libp2p::Swarm::new(transport, behaviour, libp2p::core::topology::MemoryTopology::empty(local_pub_key.clone()))
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

    println!("You can now chat with other people! Type your message and press enter.");
    // Read full lines from stdin
    let stdin = tokio_stdin_stdout::stdin(0);
    let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new()).fuse();

    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => {
                    let to_send = format!("{}> {}", nickname, line);
                    swarm.floodsub.publish(&floodsub_topic, to_send.as_bytes())
                }
                Async::Ready(None) => break, // panic!("Stdin closed"),
                Async::NotReady => break,
            };
        }

        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(message)) => {
                    // println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }));

}
