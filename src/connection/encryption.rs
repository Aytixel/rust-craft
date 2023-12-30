use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecryptMut, BlockEncryptMut, BlockSizeUser, KeyIvInit,
    },
    Aes128,
};
use anyhow::{anyhow, Result};
use boring::{
    pkey::Private,
    rsa::{Padding, Rsa},
};
use cfb8::{Decryptor, Encryptor};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct RsaEncryptor {
    rsa: Rsa<Private>,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl RsaEncryptor {
    pub fn new() -> Result<Self> {
        let rsa = Rsa::generate(1024)
            .map_err(|error| anyhow!("Failed to generate a private key : {error}"))?;

        Ok(Self {
            public_key: rsa
                .public_key_to_der()
                .map_err(|error| anyhow!("Failed to convert public key to DER format : {error}"))?,
            rsa,
            verify_token: thread_rng().gen::<[u8; 4]>().into(),
        })
    }

    pub fn decrypt(&self, from: &Vec<u8>, to: &mut Vec<u8>) -> Result<usize> {
        if to.len() < self.rsa.size() as usize {
            to.resize(self.rsa.size() as usize, 0);
        }

        self.rsa
            .private_decrypt(from, to, Padding::PKCS1)
            .map_err(|error| anyhow!("Decryption error : {error}"))
    }

    pub fn aes_from_secret(&self, shared_secret: &Vec<u8>) -> Result<(AesEncryptor, AesDecryptor)> {
        let mut key = Vec::new();
        let length = self.decrypt(shared_secret, &mut key)?;
        let key = &key[..length];

        Ok((
            AesEncryptor(
                Encryptor::new_from_slices(key, key)
                    .map_err(|error| anyhow!("Can't initilize Aes/CFB8 encryptor : {error}"))?,
            ),
            AesDecryptor(
                Decryptor::new_from_slices(key, key)
                    .map_err(|error| anyhow!("Can't initilize Aes/CFB8 decryptor : {error}"))?,
            ),
        ))
    }
}

pub struct AesEncryptor(Encryptor<Aes128>);

impl AesEncryptor {
    pub fn encrypt(&mut self, buffer: &mut [u8]) {
        for chunk in buffer.chunks_mut(Encryptor::<Aes128>::block_size()) {
            let generic_array = GenericArray::from_mut_slice(chunk);

            self.0.encrypt_block_mut(generic_array);
        }
    }
}

pub struct AesDecryptor(Decryptor<Aes128>);

impl AesDecryptor {
    pub fn decrypt(&mut self, buffer: &mut [u8]) {
        for chunk in buffer.chunks_mut(Decryptor::<Aes128>::block_size()) {
            let generic_array = GenericArray::from_mut_slice(chunk);

            self.0.decrypt_block_mut(generic_array);
        }
    }
}
