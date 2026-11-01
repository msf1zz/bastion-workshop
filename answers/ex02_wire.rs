//! Lab 2, finished. Try the exercise first.

use bastion_engine::{
    spdm_types::{SessionId, SpdmMessage, SpdmRequest, SpdmVersion},
    zc_serial::{SpdmDecode, SpdmEncode},
};

const SESSION_ID: SessionId = SessionId(0x0000_1234);

fn main() {
    let request = SpdmMessage::Request(SpdmRequest {
        version: SpdmVersion::V1_1,
        session_id: SESSION_ID,
        payload: b"ping",
    });

    // TODO 1
    let mut buf = [0u8; 64];
    let n = match request.encode(&mut buf) {
        Some(n) => n,
        None => {
            eprintln!("buffer too small for frame");
            return;
        }
    };
    println!("[wire]  encoded {} bytes: {:02x?}", n, &buf[..n]);

    // TODO 2
    let (decoded, rest) = match SpdmMessage::decode(&buf[..n]) {
        Some(pair) => pair,
        None => {
            eprintln!("malformed frame");
            return;
        }
    };
    if let SpdmMessage::Request(req) = decoded {
        println!(
            "[wire]  decoded request for session {:#010x}, payload {:?} ({} bytes left over)",
            req.session_id.0,
            String::from_utf8_lossy(req.payload),
            rest.len()
        );
    }

    // TODO 3
    let foreign = SpdmMessage::Request(SpdmRequest {
        version: SpdmVersion::V1_1,
        session_id: SessionId(0xDEAD),
        payload: b"intruder",
    });
    let n = foreign.encode(&mut buf).expect("encode");
    let (decoded, _) = SpdmMessage::decode(&buf[..n]).expect("decode");
    let frame_session = match decoded {
        SpdmMessage::Request(r) => r.session_id,
        SpdmMessage::Response(r) => r.session_id,
        SpdmMessage::Handshake(h) => h.session_id,
    };
    if frame_session != SESSION_ID {
        println!("[guard] rejected frame from session {:#010x}", frame_session.0);
    }
}
