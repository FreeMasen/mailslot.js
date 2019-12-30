#[macro_use]
extern crate neon;
extern crate mail_slot;
use mail_slot::{MailslotName as _Name, MailslotServer as _Server, MailslotClient as _Client};

use neon::prelude::*;

pub struct MailSlotServer(_Server);
pub struct MailSlotClient(_Client);

declare_types! {
    pub class JsMailSlotServer for MailSlotServer {
        init(mut cx) {
            let name = if cx.len() < 2 {
                let first = cx.argument::<JsString>(0)?.value();
                _Name::local(&first)
            } else {
                let first = cx.argument::<JsString>(0)?.value();
                let second = cx.argument::<JsString>(1)?.value();
                _Name::network(&first, &second)
            };
            let inner = _Server::new(&name).unwrap();
            Ok(MailSlotServer(inner))
        }
        method get_next_unread(mut cx) {
            let mut this = cx.this();
            let guard = cx.lock();
            let bytes = this.borrow_mut(&guard).0.get_next_unread().unwrap();
            if let Some(bytes) = bytes {
                let s = String::from_utf8(bytes).unwrap();
                Ok(cx.string(&s).upcast())
            } else {
                Ok(cx.undefined().upcast())
            }
        }
    }
    pub class JsMailSlotClient for MailSlotClient {
        init(mut cx) {
            let name = if cx.len() < 2 {
                let first = cx.argument::<JsString>(0)?.value();
                _Name::local(&first)
            } else {
                let first = cx.argument::<JsString>(0)?.value();
                let second = cx.argument::<JsString>(1)?.value();
                _Name::network(&first, &second)
            };
            let inner = _Client::new(&name).unwrap();
            Ok(MailSlotClient(inner))
        }
        method send_message(mut cx) {
            let s = cx.argument::<JsString>(0)?.value();
            let mut this = cx.this();
            let guard = cx.lock();
            this.borrow_mut(&guard).0.send_message(s.as_bytes()).unwrap();
            Ok(cx.undefined().upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsMailSlotServer>("MailSlotServer")?;
    cx.export_class::<JsMailSlotClient>("MailSlotClient")?;
    Ok(())
});
