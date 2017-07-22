extern crate extprim;
#[macro_use] extern crate extprim_literals;
extern crate hyper;
extern crate telegram;

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use hyper::Client;
use hyper::client::Body;
use hyper::header::{ContentLength, Connection};
use telegram::ser::Serialize;


fn main() {
    // Request for (p,q) Authorization
    // https://core.telegram.org/mtproto/samples-auth_key

    // [DEBUG] Step
    println!(" * Request for (p,q) Authorization");

    let req_pq = telegram::schema::mtproto::req_pq {
        nonce: i128!(0x3E0549828CCA27E966B301A48FECE2FC),
    };

    // [DEBUG] Step
    println!(" - Message");
    println!("{:?}\n", req_pq);

    let mut buffer = Vec::new();

    // auth_key_id
    0u64.serialize_to(&mut buffer).unwrap();

    // message_id
    let now_d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now_s = now_d.as_secs();
    let message_id = ((now_s as u64) << 32) + (now_d.subsec_nanos() as u64);

    message_id.serialize_to(&mut buffer).unwrap();

    // Prepare message to compute message_length
    let mut message = Vec::new();
    req_pq.serialize_to(&mut message).unwrap();

    // message_length
    (message.len() as u32).serialize_to(&mut buffer).unwrap();

    // Push the message into the buffer
    buffer.extend(message);

    // [DEBUG] Step
    println!(" - Serialize");

    // [DEBUG] Show buffer
    pprint(&buffer);

    // [DEBUG] Step
    println!(" - Send {}", "http://149.154.167.50:443/api");

    let client = Client::new();
    let mut res = client.post("http://149.154.167.50:443/api")
        .header(Connection::keep_alive())
        .header(ContentLength(buffer.len() as u64))
        .body(Body::BufBody(&buffer, buffer.len()))
        .send().unwrap();

    // [DEBUG] Show response
    println!("{}\n", res.status);
    println!("{}", res.headers);

    // [DEBUG] Step
    println!(" - Receive");

    let mut res_buffer = Vec::new();
    res.read_to_end(&mut res_buffer).unwrap();

    pprint(&res_buffer);
}

fn pprint(buffer: &[u8]) {
    const CHUNK_SIZE: usize = 0x10;

    for (index, chunk) in buffer.chunks(CHUNK_SIZE).enumerate() {
        print!(" {:04X} |", index * CHUNK_SIZE);

        for byte in chunk {
            print!(" {:02X}", byte);
        }

        println!();
    }

    println!();
}
