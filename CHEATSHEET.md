# cheatsheet — print me

## the state machine

```
+--------+   negotiate    +-------------+  authenticate   +---------------+
|  Init  | -------------> | Negotiated  | --------------> | Authenticated |
+--------+  (cipher,hash) +-------------+ (cert, sig)     +---------------+
                                                                 |
                                                          into_session()
                                                                 v
                                                          +-------------+
                                                          |   Session   |  encode / receive
                                                          +-------------+
```

Each `->` consumes the old state. The old one cannot be used again.

## the wire frame (8-byte header + payload)

```
version (1) | kind (1) | session_id (4, LE) | payload_len (2, LE) | payload...
```

kinds: 1 = request, 2 = response, 3 = handshake. max payload: 65535 bytes.

## api map

```rust
BastionHandshake::new(version, session_id)      // -> Init
    .negotiate(NegotiationProposal { cipher_suite_id, hash_algo_id })
                                                // Init -> Negotiated
    .authenticate(AuthenticationEvidence { cert_chain, signature })
                                                // Negotiated -> Authenticated
    .into_session(PeerInfo::new(id, endpoint), CryptoState::new(c, h))
                                                // -> BastionSession

session.encode_request(payload, &mut buf)       // Result<usize>
session.receive(&buf)                           // Result<(SpdmMessage, rest)>
session.terminate()                             // consumes the session
```

## sim commands

```text
cargo run --bin bastion-sim -- responder --bind 127.0.0.1:7701
cargo run --bin bastion-sim -- requester --connect 127.0.0.1:7701 [--tamper]
```

## if you fall behind

```text
cp answers/ex01_handshake.rs src/bin/ex01.rs
cp answers/ex02_wire.rs      src/bin/ex02.rs
```
