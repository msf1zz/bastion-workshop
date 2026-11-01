//! Lab 1, finished. Try the exercise first.

use bastion_engine::{
    bastion::BastionHandshake,
    handshake::{AuthenticationEvidence, NegotiationProposal},
    session::{CryptoState, PeerInfo},
    spdm_types::{SessionId, SpdmVersion},
};

const SESSION_ID: SessionId = SessionId(0x0000_1234);
const CIPHER_SUITE_ID: u16 = 0x0040;
const HASH_ALGO_ID: u16 = 0x0004;

const CERT_CHAIN: &[u8] = b"workshop-cert-chain";
const SIGNATURE: &[u8] = b"workshop-signature";

fn main() {
    let handshake = BastionHandshake::new(SpdmVersion::V1_1, SESSION_ID);
    println!("[state] Init (session {:#010x})", handshake.session_id().0);

    // TODO 1
    let handshake = handshake.negotiate(NegotiationProposal {
        cipher_suite_id: CIPHER_SUITE_ID,
        hash_algo_id: HASH_ALGO_ID,
    });
    println!("[state] Negotiated {:?}", handshake.negotiated_algorithms());

    // TODO 2
    let handshake = handshake.authenticate(AuthenticationEvidence {
        cert_chain: CERT_CHAIN,
        signature: SIGNATURE,
    });
    println!("[state] Authenticated");

    // TODO 3
    let session = handshake.into_session(
        PeerInfo::new(b"peer-01", b"tcp://127.0.0.1:7701"),
        CryptoState::new(CIPHER_SUITE_ID, HASH_ALGO_ID),
    );
    println!("[state] Session active");

    // CHECKPOINT
    let mut buf = [0u8; 64];
    let n = session.encode_request(b"ping", &mut buf).expect("encode");
    println!("[wire]  encoded {} bytes: {:02x?}", n, &buf[..n]);
}
