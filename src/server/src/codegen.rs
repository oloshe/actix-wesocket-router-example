///  注册路由
macro_rules! register_macro {
	($(
		$(#[$outer:meta])*
        $cmd:ident$(,)?)*
    ) => {
        use std::collections::HashMap;
        use actix::{Addr, Message};
        use crate::cmd::Cmd;
        use crate::ws_conn::WsConn;

        $(
            #[derive(Message)]
            #[rtype(result = "()")]
            $(#[$outer])*
            pub struct $cmd {
                // 来自于哪个模块
                pub cmd: &'static str,
                // 接受自玩家的原始字符串
                pub data: String,
            }
            impl Cmd for $cmd {
                fn name(&self) -> &'static str { stringify!($cmd) }
                fn route(addr: Addr<WsConn>, data: String) {
                    addr.do_send(Self {
                        cmd: stringify!($cmd),
                        data: data,
                    })
                }
            }
        )*
        
        lazy_static::lazy_static! {
            pub static ref CMD_MAP: HashMap<&'static str, fn(addr: Addr<WsConn>, data: String)> = {
                let mut map = HashMap::new();
                $(map.insert(stringify!($cmd), $cmd::route as fn(addr: Addr<WsConn>, data: String));)*
                map
            };
        }
    }
}