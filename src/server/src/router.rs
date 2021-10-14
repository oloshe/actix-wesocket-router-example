register_macro! {
    /** 说你好 */
    SayHello,
    /** 说话 */
    Say,
}

// use actix::Message;

// use crate::cmds::Cmd;

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct SayHello {
//     pub from: &'static str,
//     pub data: String,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Say {
//     pub from: &'static str,
//     pub data: String,
// }


// impl Cmd for SayHello {
//     fn name(&self) -> &'static str {
//         "SayHello"
//     }
//     fn route(addr: actix::Addr<crate::ws_conn::WsConn>, data: String) {
//         addr.do_send(Self { from: "SayHello", data })
//     }
// }
// impl Cmd for Say {
//     fn name(&self) -> &'static str {
//         "Say"
//     }
//     fn route(addr: actix::Addr<crate::ws_conn::WsConn>, data: String) {
//         addr.do_send(Self { from: "Say", data })
//     }
// }
