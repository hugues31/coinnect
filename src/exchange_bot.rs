use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use actix::{Context, io::SinkWrite, Actor, Handler, StreamHandler, AsyncContext, ActorContext, Addr, SystemService, Recipient};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    Client, BoxedSocket,
};
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use actix_rt::{System, Arbiter};
use std::time::Duration;
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};
use bytes::Buf;
use serde_json::{Value, Map};
use std::collections::HashMap;
use crate::helpers;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ExchangeBot {
    inner: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
    handler: Box<ExchangeBotHandler>,
}

pub trait ExchangeBotHandler {
    /// Handle incoming messages
    fn handle_in(&mut self, w: &mut SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>, msg: Bytes);
    fn handle_started(&mut self, w: &mut SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);
}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

impl Actor for ExchangeBot
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");
    }
}

impl actix::Supervised for ExchangeBot {
    fn restarting(&mut self, ctx: &mut Context<ExchangeBot>) {
        println!("restarting exchange bot...");
    }
}

impl ExchangeBot
{
    pub async fn new(wss_url: &str, handler: Box<ExchangeBotHandler>) -> Addr<ExchangeBot> {
        let c = helpers::new_ws_client(wss_url).await;
        let (sink, stream) = c.split();
        actix::Supervisor::start(|ctx| {
            ExchangeBot::add_stream(stream, ctx);
            ExchangeBot { inner: SinkWrite::new(sink, ctx), handler }
        })
    }
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.inner.write(Message::Ping(Bytes::from_static(b""))).unwrap();
            act.hb(ctx);
            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for ExchangeBot
{
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.inner.write(Message::Text(msg.0)).unwrap();
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for ExchangeBot
{
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            println!("{:?}", txt);
            self.handler.handle_in(&mut self.inner, txt);
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
        self.handler.handle_started(&mut self.inner);
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for ExchangeBot
{}

