// Copyright 2015-2016 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

#[macro_use]
extern crate aerospike;
extern crate env_logger;

use aerospike::{Client, Host};
use aerospike::{ClientPolicy, ReadPolicy, WritePolicy};
use aerospike::{Key, Bin};
use aerospike::{Operation};
use aerospike::value::*;

// use log::LogLevel;
// use env_logger;

// use std::collections::{HashMap, VecDeque};
use std::sync::{RwLock, Arc, Mutex};
// use std::vec::Vec;
use std::thread;
use std::time::{Instant, Duration};


#[test]
fn connect() {
    env_logger::init().unwrap();

    let cpolicy = ClientPolicy::default();
    let client: Arc<Client> = Arc::new(Client::new(&cpolicy, &vec![Host::new("ubvm", 3000)]).unwrap());

    let t: i64 = 1;
    let key = Key::new("ns", "set", Value::from(t));
    let key = key!("ns", "set", t);
    let key = key!("ns", "set", &t);
    let key = key!("ns", "set", 1);
    let key = key!("ns", "set", &1);
    let key = key!("ns", "set", 1i8);
    let key = key!("ns", "set", &1i8);
    let key = key!("ns", "set", 1u8);
    let key = key!("ns", "set", &1u8);
    let key = key!("ns", "set", 1.0f32);
    let key = key!("ns", "set", &1.0f32);
    let key = key!("ns", "set", 1.0f64);
    let key = key!("ns", "set", &1.0f64);


    let mut threads = vec![];
    let now = Instant::now();
    for _ in 0..2 {
    	let client = client.clone();
	    let t = thread::spawn(move || {
		    let policy = ReadPolicy::default();

		    let wpolicy = WritePolicy::default();
		    let key = key!("test", "test", -1);
		    let wbin = bin!("bin999", 1);
		    let bins = vec![&wbin];

			client.put(&wpolicy, &key, &bins).unwrap();
		    let rec = client.get(&policy, &key, None);
		    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {}", rec.unwrap());

			client.touch(&wpolicy, &key).unwrap();
		    let rec = client.get(&policy, &key, None);
		    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {}", rec.unwrap());

		    let rec = client.get_header(&policy, &key);
		    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@/// {}", rec.unwrap());

			let exists = client.exists(&wpolicy, &key).unwrap();
			println!("exists: {}", exists);

			let ops = &vec![Operation::put(&wbin), Operation::get()];
			let op_rec = client.operate(&wpolicy, &key, ops);
			println!("operate: {}", op_rec.unwrap());

			let existed = client.delete(&wpolicy, &key).unwrap();
			println!("existed: {}", existed);

			let existed = client.delete(&wpolicy, &key).unwrap();
			println!("existed: {}", existed);
		});
		threads.push(t);
	}

	for t in threads {
		t.join();
	}
	println!("total time: {:?}", now.elapsed());

    let wpolicy = WritePolicy::default();
    let key = key!("test", "test", -1);
    let wbin = bin!("bin666", -1);
    let bins = vec![&wbin];
	client.put(&wpolicy, &key, &bins).unwrap();

    let now = Instant::now();
    let mut threads = vec![];
    for _ in 0..16 {
    	let client = client.clone();
	    let t = thread::spawn(move || {
		    let policy = ReadPolicy::default();
		    let key = key!("test", "test", -1);
		    for i in 1..10_000 {
			    let rec = client.get(&policy, &key, None);
			    // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {}", rec.unwrap());
			}
		});
		threads.push(t);
	}

	for t in threads {
		t.join();
	}

	println!("total time: {:?}", now.elapsed());

	struct T(i64);
	struct TN{N: i64};

    let now = Instant::now();
	for _ in 0..10_000_000 {
		    let wbin = 1;
	}
	println!("total time: {:?}", now.elapsed());

    let now = Instant::now();
	for _ in 0..10_000_000 {
		    let wbin = T(1);
	}
	println!("total time: {:?}", now.elapsed());

    let now = Instant::now();
	for _ in 0..10_000_000 {
		    let wbin = TN{N:1};
	}
	println!("total time: {:?}", now.elapsed());

 //    let now = Instant::now();
	// for _ in 0..10_000_000 {
	// 	    let wbin = Box::new(1);
	// }
	// println!("total time: {:?}", now.elapsed());

 //    for _ in 1..100 {
 //        let cluster = client.cluster.clone();
 //        println!("{:?}", cluster.nodes().len());
 //        thread::sleep(Duration::from_millis(1000));
 //    }
 //    assert_eq!(2, 2);
}
