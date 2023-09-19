use std::io::{Cursor, Read};
use m3u8_rs::{Key as m3u8Key, KeyMethod};
use aesstream::AesReader;
use serde::{Serialize, Deserialize};
use url::Url;

use super::util::download;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum KeyType {
    None,
    Aes128,
    SampleAES
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct M3u8EncryptKey {
    pub ty: KeyType,
    iv: Option<String>,
    content: Vec<u8>
}

impl Default for M3u8EncryptKey {
    fn default() -> Self {
        M3u8EncryptKey {
            ty: KeyType::None,
            iv: None,
            content: vec![]
        }
    }
}

impl M3u8EncryptKey {
    pub async fn from_key(base_url: Url, k: &m3u8Key) -> anyhow::Result<Self> {
        let key_url = base_url.join(k.uri.as_ref().unwrap()).unwrap();
        Ok(match k.method {
            KeyMethod::None => {
                M3u8EncryptKey {
                    ty: KeyType::None,
                    iv: k.iv.clone(),
                    content: vec![]
                }
            },
            KeyMethod::AES128 => {
                M3u8EncryptKey {
                    ty: KeyType::Aes128,
                    iv: k.iv.clone(),
                    content: download(&key_url).await?
                }
            }
            KeyMethod::SampleAES => {
                M3u8EncryptKey {
                    ty: KeyType::SampleAES,
                    iv: k.iv.clone(),
                    content: download(&key_url).await?
                }
            }
            _ => panic!("{}", format!("Unsupported key method: {}", &k.method))
        })
    }

    pub fn decode(&self, data: &[u8]) -> Option<Vec<u8>> {
        let decryptor = crypto::aessafe::AesSafe128Decryptor::new(&self.content);
        let mut reader = AesReader::new(Cursor::new(data), decryptor).unwrap();
        let mut v = Vec::new();
        match reader.read_to_end(&mut v) {
            Ok(_p) => Some(v),
            Err(_) => None
        }
    }
}