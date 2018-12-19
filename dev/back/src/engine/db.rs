// local db for list of trusted friends

extern crate rocksdb;

use rocksdb::DB;
use rand::prelude::*;
use std::path::PathBuf;
use std::collections::*;

static DBNAME: &str = "molvadb";

pub struct MolvaDB {
    db_name: String,
    db_path: String,
}

impl MolvaDB {

    pub fn new(db_name: String, db_path: String) -> Self {
        Self {
            db_name,
            db_path,
        }
    }

    pub fn open(path: String) -> DB {
        DB::open_default(path).unwrap()
    }

    pub fn add_friend(&self, ip4_addr: usize, ip6_addr: usize, name: String) {
        let uuid = rand::random();
        let friend = Friend::new(uuid, ip4_addr, ip6_addr, name);
        DB::open_default(DBNAME).unwrap().put(b"uuid", b"friend"); // how to convert usize and struct Friend  to &[u8]? uuid to string, friend to string
    }

    pub fn get_by_name() -> Friend {
        return Friend::new(0, 0, 0, "test".to_string());
    }



    // get by ip4, get by ip6, get by ???
    // get public node for peer discovery


//    match db.get(b"my key") {
//    Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
//    Ok(None) => println!("value not found"),
//    Err(e) => println!("operational problem encountered: {}", e),
//    }
//    db.delete(b"my key").unwrap();
}

pub struct Friend {
    uuid: usize,
    // UUID in DB
    ip4_addr: usize,
    // User's IP4 address
    ip6_addr: usize,
    // User's IP6 address
    name: String, // User's name
}

impl Friend {
    fn new(uuid: usize, ip4_addr: usize, ip6_addr: usize, name: String) -> Self {
        Self {
            uuid,
            ip4_addr,
            ip6_addr,
            name,
        }
    }
}
