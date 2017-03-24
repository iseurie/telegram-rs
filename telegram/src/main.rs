// // // #![feature(i128_type)]

// // // extern crate telegram;
// // // extern crate serde;

// // // #[macro_use]
// // // extern crate serde_derive;

// // // use std::mem::size_of;
// // // use std::io::Cursor;
// // // use serde::{Serialize, Deserialize, Deserializer};
// // // use serde::ser::{SerializeStruct, SerializeTuple};

// // // // Exports `Serialize` and `Deserialize` procedural derive macros
// // // // telegram_derive

// // // // telegram_codegen

// extern crate telegram_codegen;
// use std::path::Path;

// fn main() {
//     telegram_codegen::translate("schema/mtproto-schema.json", &Path::new("schema.rs")).unwrap();
// }

// // // // Path: /api
// // // // [..] they are made into a payload which is followed by a POST request to the URL/api [..]
// // // // https://core.telegram.org/mtproto#http-transport

// // // // #[derive(Serialize, Deserialize, Debug, Clone)]
// // // // #[id(85337187)]
// // // struct ResPQ {
// // //     nonce: i128,
// // //     server_nonce: i128,
// // //     pq: Vec<u8>,
// // //     server_public_key_fingerprints: Vec<i64>,
// // // }

// // // pub fn req_pq(nonce: i128) -> Result<()> {
// // //     // #[derive(Serialize)]
// // //     // #[id(1615239032)]
// // //     struct req_pq {
// // //         nonce: i128,
// // //     }

// // //     let data = req_pq { nonce: nonce };

// // //     // A message is ::
// // //     //  auth_key_id: u64
// // //     //  message_id: u64
// // //     //  message_length: u32
// // //     //  data: ...

// // //     let mut body = Vec::<u8>::new();

// // //     // Key Identifier
// // //     // [auth_key_id: u64]
// // //     0u64.serialize(&mut body);

// // //     // Compute message identifier
// // //     let now_d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
// // //     let now_s = now_d.as_secs();
// // //     let message_id = ((now_s as u64) << 32) + (now_d.subsec_nanos() as u64);

// // //     // Message Identifier
// // //     // [message_id: u64]
// // //     message_id.serialize(&mut body);

// // //     // Compute message length
// // //     // <identifier> (4) + <data size>
// // //     let message_data_length = 4 + (size_of::<req_pq>()) as u32;

// // //     // Message Length
// // //     message_data_length.serialize(&mut body);

// // //     // Message Data

// // //     // Identifier
// // //     1615239032i32.serialize(&mut body);

// // //     // ***
// // //     data.nonce.serialize(&mut body);

// // //     let client = Client::new();
// // //     let mut res = client.post("http://149.154.167.50:443/api")
// // //         .header(Connection::keep_alive())
// // //         .header(ContentLength(body.len() as u64))
// // //         .body(Body::BufBody(&body, body.len()))
// // //         .send()?;

// // //     println!("status: {}", res.status);
// // //     println!("headers: {}", res.headers);
// // //     println!("\n-----\n{:?}", res);

// // //     let mut res_body = Vec::new();
// // //     res.read_to_end(&mut res_body)?;

// // //     let mut f = File::create("foo.txt")?;
// // //     f.write_all(&res_body)?;

// // //     Ok(())
// // // }


// extern crate byteorder;

// use std::error::Error;
// use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

// trait Serialize {
//     fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<(), Box<Error>>;
// }

// impl Serialize for String {
//     fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<(), Box<Error>> {
//         let len = self.len();

//         if len <= 253 {
//             // If L <= 253, the serialization contains one byte with the value of L,
//             // then L bytes of the string followed by 0 to 3 characters containing 0,
//             // such that the overall length of the value be divisible by 4,
//             // whereupon all of this is interpreted as a sequence
//             // of int(L/4)+1 32-bit little-endian integers.

//             buffer.push(len as u8);
//         } else {
//             // If L >= 254, the serialization contains byte 254, followed by 3
//             // bytes with the string length L in little-endian order, followed by L
//             // bytes of the string, further followed by 0 to 3 null padding bytes.

//             buffer.push(254);
//             buffer.write_uint::<LittleEndian>(len as u64, 3)?;
//         }

//         // Write each character in the string
//         buffer.extend(self.as_bytes());

//         // [...] string followed by 0 to 3 characters containing 0,
//         // such that the overall length of the value be divisible by 4 [...]
//         let rem = len % 4;
//         if rem > 0 {
//             for _ in 0..(4 - rem) {
//                 buffer.push(0);
//             }
//         }

//         Ok(())
//     }
// }

// #[derive(Debug)]
// // #[id = "-1527411636"]
// struct SentChangePhoneCode {
//     phone_code_hash: String,
//     send_call_timeout: i32,
// }

// impl SentChangePhoneCode {
//     fn serialize(&self, buffer: &mut Vec<u8>) {
//         buffer.write_i32::<LittleEndian>(-1527411636).unwrap();

//         self.phone_code_hash.serialize_to(buffer).unwrap();
//         // self.send_call_timeout.serialize_to(buffer).unwrap();
//     }
// }

// fn main() {
//     let rec = SentChangePhoneCode {
//         phone_code_hash: "abcdefgh".into(),
//         send_call_timeout: 1204,
//     };

//     let mut buffer = Vec::new();
//     rec.serialize(&mut buffer);

//     println!("{:?}", buffer);
// }
