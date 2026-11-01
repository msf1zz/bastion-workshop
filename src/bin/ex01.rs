//! Lab 1. Walk a device from Init to a real session.
//! Skip a step and it will not compile.
//!
//! cargo run --bin ex01
//! stuck? cp answers/ex01_handshake.rs src/bin/ex01.rs

use bastion_engine::{
    bastion::BastionHandshake,
    handshake::{AuthenticationEvidence, NegotiationProposal},
    session::{CryptoState, PeerInfo},
    spdm_types::{SessionId, SpdmVersion},
};

const SESSION_ID: SessionId = SessionId(0x0000_1234);
const CIPHER_SUITE_ID: u16 = 0x0040;
const HASH_ALGO_ID: u16 = 0x0004;

// fake cert bytes. the engine just stores them.
const CERT_CHAIN: &[u8] = b"workshop-cert-chain";
const SIGNATURE: &[u8] = b"workshop-signature";

fn main() {
    // already done: every connection starts here.
    let handshake = BastionHandshake::new(SpdmVersion::V1_1, SESSION_ID);
    println!("[state] Init (session {:#010x})", handshake.session_id().0);

    // TODO 1: agree on algorithms. negotiate() eats the old handshake, so rebind it.
    // let handshake = handshake.negotiate(NegotiationProposal { cipher_suite_id, hash_algo_id });
    todo!("TODO 1: negotiate algorithms");

    // TODO 2: prove who you are. pass AuthenticationEvidence { cert_chain, signature }.
    // todo!("TODO 2: authenticate");

    // TODO 3: turn that into a session you can actually send on.
    // peer = PeerInfo::new(b"peer-01", b"tcp://127.0.0.1:7701")
    // crypto = CryptoState::new(CIPHER_SUITE_ID, HASH_ALGO_ID)
    // todo!("TODO 3: promote to a session");

    // once TODO 3 works, send something:
    // let mut buf = [0u8; 64];
    // let n = session.encode_request(b"ping", &mut buf).expect("encode");
    // println!("[wire] encoded {n} bytes: {:02x?}", &buf[..n]);
    //
    // then break it on purpose, one line at a time, and read the error:
    // a) handshake.encode_request(...)  -> no encode method yet
    // b) Init.authenticate(...)          -> you can't skip negotiate
    // c) session.terminate(), then use session again -> it's gone
}
