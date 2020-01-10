// This example shows how to use the generic API provided by Coinnect.
// This method is useful if you have to iterate throught multiple accounts of
// different exchanges and perform the same operation (such as get the current account's balance)
// You can also use the Coinnect generic API if you want a better error handling since all methods
// return Result<_, Error>.

extern crate coinnect;
extern crate actix;
#[macro_use] extern crate actix_derive;

use crate::coinnect::coinnect_rt::Coinnect;
use crate::coinnect::bitstamp::BitstampCreds;
use crate::coinnect::exchange::Exchange::*;
use crate::coinnect::types::Pair::*;
use std::path::PathBuf;
use coinnect_rt::helpers;
use serde::{Serialize, Deserialize};
use actix::{Context, io::SinkWrite, Actor, Handler, StreamHandler, AsyncContext, ActorContext};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    Client, BoxedSocket
};
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use actix_rt::{System, Arbiter};
use std::time::Duration;
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};
use bytes::Buf;
use serde_json::{Value, Map};
use std::collections::HashMap;

fn main() {
    let sys = System::new("websocket-client");
    Arbiter::spawn(async {
        let path = PathBuf::from("keys_real.json");
        let my_creds = BitstampCreds::new_from_file("account_bitstamp", path).unwrap();
        coinnect_rt::new_stream(Bitstamp, my_creds, None).await;
    });
    sys.run().unwrap();
}
