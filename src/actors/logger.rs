
use actix::prelude::*;

use crate::common::ReceivedLine;

pub struct ConsoleLogger;

impl Actor for ConsoleLogger {
    type Context = Context<Self>;
}

impl Handler<ReceivedLine> for ConsoleLogger {
    type Result = ();

    fn handle(&mut self, message: ReceivedLine, _ctx: &mut Context<Self>) {
        println!("{}", message.line);
    }
}