use core::marker::PhantomData;

use aead::{
    AeadCore, AeadInPlace, Error, KeyInit, KeySizeUser,
    consts::{U0, U12, U16, U24, U32},
    generic_array::ArrayLength,
};
use nrfxlib_sys::{ocrypto_aes_gcm_decrypt, ocrypto_aes_gcm_encrypt};
use zeroize::Zeroize;

pub const ASSOCIATED_DATA_MAX: u64 = 1 << 36;
pub const PLAINTEXT_MAX: u64 = 1 << 36;
pub const CIPHERTEXT_MAX: u64 = (1 << 36) + 16;

pub type Aes128Gcm = AesGcm<U16>;
pub type Aes196Gcm = AesGcm<U24>;
pub type Aes256Gcm = AesGcm<U32>;

#[derive(Zeroize)]
pub struct AesGcm<Size: AesSize> {
    key: aead::Key<Self>,
    _marker: PhantomData<Size>,
}

impl<Size: AesSize> KeySizeUser for AesGcm<Size> {
    type KeySize = Size;
}

impl<Size: AesSize> KeyInit for AesGcm<Size> {
    fn new(key: &aead::Key<Self>) -> Self {
        Self {
            key: key.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Size: AesSize> AeadCore for AesGcm<Size> {
    type NonceSize = U12;
    type TagSize = U16;
    type CiphertextOverhead = U0;
}

impl<Size: AesSize> AeadInPlace for AesGcm<Size> {
    fn encrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> aead::Result<aead::Tag<Self>> {
        if buffer.len() as u64 > PLAINTEXT_MAX || associated_data.len() as u64 > ASSOCIATED_DATA_MAX
        {
            return Err(Error);
        }

        let mut tag = aead::Tag::<Self>::default();

        // SAFETY: key size is 16, 24 or 32 as ensured by the [`AesSize`] trait
        // SAFETY: nonce size is 12 as ensured by the implementation of the [`AeadCore`] trait
        // SAFETY: tag size is 16 as ensured by the implementation of the [`AeadCore`] trait
        // SAFETY: it is valid to pass a length of 0
        unsafe {
            ocrypto_aes_gcm_encrypt(
                buffer.as_mut_ptr(),
                tag.as_mut_ptr(),
                tag.len(),
                buffer.as_ptr(),
                buffer.len(),
                self.key.as_ptr(),
                self.key.len(),
                nonce.as_ptr(),
                associated_data.as_ptr(),
                associated_data.len(),
            );
        }

        Ok(tag)
    }

    fn decrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &aead::Tag<Self>,
    ) -> aead::Result<()> {
        if buffer.len() as u64 > CIPHERTEXT_MAX
            || associated_data.len() as u64 > ASSOCIATED_DATA_MAX
        {
            return Err(Error);
        }

        // SAFETY: key size is 16, 24 or 32 as ensured by the [`AesSize`] trait
        // SAFETY: nonce size is 12 as ensured by the implementation of the [`AeadCore`] trait
        // SAFETY: tag size is 16 as ensured by the implementation of the [`AeadCore`] trait
        // SAFETY: it is valid to pass a length of 0
        let res = unsafe {
            ocrypto_aes_gcm_decrypt(
                buffer.as_mut_ptr(),
                tag.as_ptr(),
                tag.len(),
                buffer.as_ptr(),
                buffer.len(),
                self.key.as_ptr(),
                self.key.len(),
                nonce.as_ptr(),
                associated_data.as_ptr(),
                associated_data.len(),
            )
        };

        if res != 0 {
            return Err(Error);
        }

        Ok(())
    }
}

impl AesSize for U32 {}
impl AesSize for U16 {}
impl AesSize for U24 {}

pub trait AesSize: ArrayLength<u8> + private::Sealed {}

mod private {
    use aead::consts::{U16, U24, U32};

    pub trait Sealed {}
    impl Sealed for U32 {}
    impl Sealed for U16 {}
    impl Sealed for U24 {}
}
