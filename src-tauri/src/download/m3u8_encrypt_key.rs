use m3u8_rs::{Key as m3u8Key, KeyMethod};
use nom::{IResult, bytes::complete::tag, AsBytes};
use serde::{Serialize, Deserialize};
use url::Url;
use aes::cipher::{block_padding::Pkcs7, KeyIvInit, BlockDecryptMut};

use super::util::download_request;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

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
                    content: download_request(&key_url).await?
                }
            }
            KeyMethod::SampleAES => {
                M3u8EncryptKey {
                    ty: KeyType::SampleAES,
                    iv: k.iv.clone(),
                    content: download_request(&key_url).await?
                }
            }
            _ => panic!("{}", format!("Unsupported key method: {}", &k.method))
        })
    }

    pub fn decode(&self, data: &[u8]) -> Option<Vec<u8>> {
        if self.content.len() == 0 {
            return None;
        }
        let cipher_len = data.len();
        let mut buf = vec![0u8; cipher_len]; 
        let m3u8_key: String = String::from_utf8_lossy(&self.content).to_string();

        match &self.iv {
            Some(iv) => {
                let pt = Aes128CbcDec::new(m3u8_key.as_bytes().into(), hex::decode(get_hex(iv)).unwrap().as_bytes().into())
                .decrypt_padded_b2b_mut::<Pkcs7>(data, &mut buf)
                .unwrap();
                Some(pt.to_vec())
            },
            None => None,
        }
    }
}

fn get_hex(s: &str) -> &str {
    let (i, _) = parser_hex(s).unwrap_or((s, ""));
    i
}

fn parser_hex(s: &str) -> IResult<&str, &str> {
    tag("0x")(s)
}