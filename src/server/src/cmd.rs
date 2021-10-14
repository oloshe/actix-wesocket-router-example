use actix::Addr;
use crate::ws_conn::WsConn;

pub trait Cmd {
    /// 命令的名字
    fn name(&self) -> &'static str;
    /// 路由的方法
    fn route(addr: Addr<WsConn>, data: String);
}

// use crate::router::{Say, SayHello};
// use std::collections::HashMap;
// type RouterFn = fn(addr: Addr<WsConn>, data: String);
// lazy_static::lazy_static!{
//     pub static ref CMD_MAP: HashMap<&'static str, RouterFn> = {
//         let map = HashMap::new();
//         map.insert("SayHello", SayHello::route as RouterFn);
//         map.insert("Say", Say::route as RouterFn);
//         map
//     };
// }