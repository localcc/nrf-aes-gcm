# AES-GCM for Nordic chips

AES-GCM support for nordic chips using the [`aead`](https://docs.rs/aead/latest/aead/) crate primitives.


## Examples

```
cargo add nrf-aes-gcm --features nrf9160
```

```rust
let mut some_data = b"hello world";
let mut encrypted = heapless::Vec::<u8, 27>::from_slice(some_data).unwrap();

// GET THIS FROM A RANDOM GENERATOR! THIS IS JUST AN EXAMPLE
let nonce = [0u8; 12];

let aes = AesGcm::new(&shared_secret.into());
aes.encrypt_in_place(&nonce.into(), &[], &mut encrypted)
    .unwrap();

let mut decrypted = encrypted.clone();
aes.decrypt_in_place(&nonce.into(), &[], &mut decrypted)
    .unwrap();

let decrypted = unsafe { core::str::from_utf8_unchecked(decrypted.as_ref()) };
info!(
    "{:#?} {:#?} {:?}",
    some_data,
    encrypted.as_slice(),
    decrypted
);
```
