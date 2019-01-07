extern crate actix;
extern crate tokio_io;
extern crate tokio_codec;

use actix::prelude::*;

use sw_episode_iv::actors::client::TcpClientActor;
use sw_episode_iv::actors::logger::ConsoleLogger;

fn main() {
     let sys = actix::System::new("sw-episode-iv-sys");

    let _logger = Arbiter::start(|_| ConsoleLogger);

    let _tcp_client = Arbiter::start(|_| {
        TcpClientActor { recipient: _logger.recipient() }
    });

    sys.run();
}
