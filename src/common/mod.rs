extern crate actix;

use actix::prelude::*;

#[derive(Message)]
pub struct ReceivedLine {
    pub line: String,
}