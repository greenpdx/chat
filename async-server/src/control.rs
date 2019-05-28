#![feature(async_await, await_macro)]

// use futures::executor::{self, ThreadPool};
// use futures::future::join_all;
use futures::io::{AsyncWrite, AsyncWriteExt};
use futures::lock::Mutex;
// use futures::prelude::*;
// use futures::task::SpawnExt;
// use lines::Lines;
// use protocol::{Reply, Request};
use romio::{TcpListener, TcpStream};
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::io;
use std::marker::Unpin;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
// use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// A connection to a client, to which we can write serialized `Reply` objects.
#[derive(Clone, Debug)]
pub struct Outbound(pub Arc<Mutex<Box<dyn 'static + AsyncWrite + Send + Unpin>>>);

#[derive(Default, Debug)]
pub struct Channel {
    pub subscribers: HashMap<SocketAddr, Outbound>,
}

#[derive(Default, Debug)]
pub struct ChannelMap {
    pub channels: HashMap<String, Channel>,
}

pub async fn control(cmd: Value, stream: &TcpStream, channel_map: &Arc<Mutex<ChannelMap>>) -> Value {
    let map = await!(channel_map.lock());
    //let chanlst = map.channels.keys().borrow();
    let mut strout: Vec<String> = Vec::new();
    for key in map.channels.keys() {
        strout.push(key.clone().to_string());

        //await!(send_reply(&outbound, Reply::Control(key.to_string())));
    };
    println!("{:?}", strout );
    json!(strout)
}
