extern crate telegram;

use telegram::ser::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Request for (p,q) Authorization
    // https://core.telegram.org/mtproto/samples-auth_key

    // [DEBUG] Step
    println!(" * Request for (p,q) Authorization");

    let req_pq = telegram::schema::mtproto::req_pq {
        nonce: 0x3E0549828CCA27E966B301A48FECE2FC,
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
    (message.len() as u64).serialize_to(&mut buffer).unwrap();

    // Push the message into the buffer
    buffer.extend(message);

    // [DEBUG] Step
    println!(" - Serialize");

    // [DEBUG] Show buffer
    pprint(&buffer);
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
