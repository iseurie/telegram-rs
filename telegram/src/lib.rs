#![feature(i128_type)]
#![feature(concat_idents)]
#![feature(attr_literals)]

extern crate byteorder;

#[macro_use]
extern crate telegram_derive;

#[macro_use]
extern crate error_chain;

pub mod ser;
// pub mod de;
pub mod error;

#[allow(non_camel_case_types)]
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));

    pub mod mtproto {
        include!(concat!(env!("OUT_DIR"), "/mtproto_schema.rs"));
    }
}
