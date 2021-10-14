use actix::Handler;

use crate::{router::{Say, SayHello}, ws_conn::WsConn};

impl Handler<SayHello> for WsConn {
    type Result = ();

    fn handle(&mut self, _: SayHello, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("hello!");
    }
}

impl Handler<Say> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: Say, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.data);
    }
}