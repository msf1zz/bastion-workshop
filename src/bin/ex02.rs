//! Lab 2. Encode a message into a tiny buffer and read it back. No heap.
//!
//! header is 8 bytes, then the payload:
//! version | kind | session id (4) | length (2) | payload...
//!
//! cargo run --bin ex02
//! stuck? cp answers/ex02_wire.rs src/bin/ex02.rs

use bastion_engine::{
    spdm_types::{SessionId, SpdmMessage, SpdmRequest, SpdmVersion},
    zc_serial::{SpdmDecode, SpdmEncode},
};

const SESSION_ID: SessionId = SessionId(0x0000_1234);

fn main() {
    // already done: one request for our session.
    let request = SpdmMessage::Request(SpdmRequest {
        version: SpdmVersion::V1_1,
        session_id: SESSION_ID,
        payload: b"ping",
    });

    // TODO 1: write it into a 64-byte array. encode() returns None if it doesn't fit.
    // let mut buf = [0u8; 64];
    // then try [0u8; 4] and see why 4 is too small (header alone is 8).
    todo!("TODO 1: encode into buf, handle the None case");

    // TODO 2: read it back with SpdmMessage::decode(&buf[..n]).
    // the payload slice points into buf. nothing got copied.
    // todo!("TODO 2: decode and inspect");

    // TODO 3: make the same frame with session id 0xDEAD and reject it.
    // the real engine does this inside receive(). you're doing it by hand once.
    // todo!("TODO 3: reject a frame from the wrong session");
}
