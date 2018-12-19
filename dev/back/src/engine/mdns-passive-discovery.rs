extern crate futures;
extern crate libp2p;
extern crate rand;
extern crate tokio;

use futures::prelude::*;
use libp2p::mdns::service::{MdnsPacket, MdnsService};
use std::io;

fn main() {
    // This example provides passive discovery of the libp2p nodes on the network that send
    // mDNS queries and answers.

    // We start by creating the service.
    let mut service = MdnsService::new().expect("Error while creating mDNS service");

    // Create a never-ending `Future` that polls the service for events.
    let future = futures::future::poll_fn(move || -> Poll<(), io::Error> {
        loop {
            // Grab the next available packet from the service.
            let packet = match service.poll() {
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

    // Blocks the thread until the future runs to completion (which will never happen).
    tokio::run(future.map_err(|err| panic!("{:?}", err)));
}
