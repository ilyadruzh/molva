extern crate env_logger;
#[macro_use]
extern crate libp2p;
extern crate futures;
extern crate tokio;
extern crate rayon;
extern crate crossbeam;
#[macro_use]
extern crate log;


mod engine;

use futures::prelude::*;
use libp2p::{
    Transport, NetworkBehaviour,
    core::upgrade::{self, OutboundUpgradeExt},
    secio,
    core::swarm,
    mplex,
    tokio_codec::{FramedRead, LinesCodec},
};
use std::env;
use std::io::{self, Write};
use libp2p::mdns::service::{MdnsPacket, MdnsService};
use std::time::Duration;
use std::thread;
use rayon::prelude::*;
use std::thread::JoinHandle;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

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

    // create local DB OR open local DB..
    // let db = engine::db::MolvaDB::open("../db".to_string());
    // add test user
    // db.put(nickname.as_bytes(), local_peer_id.as_bytes());

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
            }
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    let five_seconds = Duration::new(5, 0);

    println!("You can now chat with other people! Type your message and press enter.");
    // Read full lines from stdin

    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {

        let loop_1: JoinHandle<Result<Async<MdnsPacket>, Async<MdnsPacket>>> = thread::spawn(move || {
            let mut mdns_service = MdnsService::new().expect("Error while creating mDNS service");
            loop {
                println!("loop 1");
                // Grab the next available packet from the service.
                let packet = match mdns_service.poll() {
                    Async::Ready(packet) => packet,
                    Async::NotReady => return Ok(Async::NotReady),
                };

                match packet {
                    MdnsPacket::Query(query) => {
                        // We detected a libp2p mDNS query on the network. In a real application, you
                        // probably want to answer this query by doing `query.respond(...)`.
                        println!("Detected query from {:?}", query.remote_addr());
                    }
                    MdnsPacket::Response(response) => {
                        // We detected a libp2p mDNS response on the network. Responses are for
                        // everyone and not just for the requester, which makes it possible to
                        // passively listen.
                        for peer in response.discovered_peers() {
                            println!("Discovered peer {:?}", peer.id());
                            // These are the self-reported addresses of the peer we just discovered.
                            for addr in peer.addresses() {
                                println!(" Address = {:?}", addr);
                            }
                        }
                    }
                    MdnsPacket::ServiceDiscovery(query) => {
                        // The last possibility is a service detection query from DNS-SD.
                        // Just like `Query`, in a real application you probably want to call
                        // `query.respond`.
                        println!("Detected service query from {:?}", query.remote_addr());
                    }
                }
            }
        });

        let nik = nickname.clone();

        let loop_2 = thread::spawn(move || {
            let stdin = tokio_stdin_stdout::stdin(0);
            let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new()).fuse();

            loop {
                println!("loop 2");
                match framed_stdin.poll().expect("Error while polling stdin") {
                    Async::Ready(Some(line)) => {
                        let to_send = format!("{:?}> {}", nik, line);
                        swarm.floodsub.publish(&floodsub_topic, to_send.as_bytes())
                    }
                    Async::Ready(None) => break, // panic!("Stdin closed"),
                    Async::NotReady => break,
                };
            }
        });


        loop_1.join().unwrap();
        loop_2.join().unwrap();

//        let loop_3 = thread::spawn(move || {
//
//            loop {
//                println!("loop 3");
//                match swarm.poll().expect("Error while polling swarm") {
//                    Async::Ready(Some(message)) => {
//                        // println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
//                    }
//                    Async::Ready(None) | Async::NotReady => break,
//                }
//            }
//        });
//
//        loop_3.join().unwrap();


        Ok(Async::NotReady)
    }));
}
