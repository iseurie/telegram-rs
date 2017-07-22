extern crate byteorder;
#[macro_use] extern crate error_chain;
extern crate extprim;
#[macro_use] extern crate telegram_derive;

pub mod ser;
// pub mod de;
pub mod error;

#[allow(non_camel_case_types)]
#[allow(non_shorthand_field_patterns)]
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));

    pub mod mtproto {
        include!(concat!(env!("OUT_DIR"), "/mtproto_schema.rs"));
    }
}
