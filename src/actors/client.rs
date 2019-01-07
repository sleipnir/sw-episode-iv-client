extern crate actix;
extern crate tokio_codec;
extern crate tokio_io;

use actix::prelude::*;
use actix::actors::resolver::{Connect, Resolver};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_io::AsyncRead;

use crate::common::ReceivedLine;

pub struct TcpClientActor {
    pub recipient: Recipient<ReceivedLine>,
}

impl Actor for TcpClientActor {
     type Context = Context<Self>;

     fn started(&mut self, ctx: &mut Self::Context) {
        println!("TcpClientActor started!");

        Resolver::from_registry()
            .send(Connect::host("towel.blinkenlights.nl:23"))
            .into_actor(self)
            .map(|res, _act, ctx| match res {
                Ok(stream) => {
                    println!("TcpClientActor connected!");
                    let (r, w) = stream.split();
                    let line_reader = FramedRead::new(r, LinesCodec::new());
                    ctx.add_stream(line_reader);
                }
                Err(err) => {
                    println!("TcpClientActor failed to connected: {}", err);
                    ctx.stop();
                }
            })
            .map_err(|err, _act, ctx| {
                println!("TcpClientActor failed to connected: {}", err);
                ctx.stop();
            })
            .wait(ctx);
    }
}

impl StreamHandler<String, std::io::Error> for TcpClientActor {
    fn handle(&mut self, line: String, _ctx: &mut Self::Context) {
        if let Err(error) = self.recipient.do_send(ReceivedLine { line }) {
            println!("do_send failed: {}", error);
        }
    }
}