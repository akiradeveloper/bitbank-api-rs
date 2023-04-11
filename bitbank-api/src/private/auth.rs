use reqwest::header::HeaderMap;

const ACCESS_KEY: &'static str = "ACCESS-KEY";
const ACCESS_NONCE: &'static str = "ACCESS-NONCE";
const ACCESS_SIGNATURE: &'static str = "ACCESS-SIGNATURE";

fn generate_nonce() -> u64 {
    use std::time::SystemTime;
    let now = SystemTime::now();
    let du = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    du.as_secs()
}

#[derive(Debug, Clone)]
pub struct Credential {
    pub api_key: String,
    pub api_secret: String,
}
impl Credential {
    /// Create a credential from the env variables.
    ///
    /// - BITBANK_API_KEY
    /// - BITBANK_API_SECRET
    pub fn from_env() -> anyhow::Result<Self> {
        use std::env;
        let api_key = env::var("BITBANK_API_KEY")?;
        let api_secret = env::var("BITBANK_API_SECRET")?;
        Ok(Self {
            api_key,
            api_secret,
        })
    }
    fn signature(&self, message: &str) -> String {
        use crypto::hmac::Hmac;
        use crypto::mac::Mac;
        use crypto::sha2::Sha256;
        use hex::ToHex;

        let mut mac = Hmac::new(Sha256::new(), self.api_secret.as_bytes());
        mac.input(message.as_bytes());
        let signature: String = mac.result().code().encode_hex();
        signature
    }
}

pub struct GetAuth {
    pub path: String,
    pub params: String,
}
impl GetAuth {
    pub fn create(self, cred: Credential) -> anyhow::Result<HeaderMap> {
        let mut out = HeaderMap::new();
        let nonce = generate_nonce();
        let sig = {
            let message = format!("{nonce}{}{}", self.path, self.params);
            cred.signature(&message)
        };
        out.insert(ACCESS_KEY, cred.api_key.try_into()?);
        out.insert(ACCESS_NONCE, nonce.try_into()?);
        out.insert(ACCESS_SIGNATURE, sig.try_into()?);
        Ok(out)
    }
}

pub struct PostAuth {
    pub body: String,
}
impl PostAuth {
    pub fn create(self, cred: Credential) -> anyhow::Result<HeaderMap> {
        let mut out = HeaderMap::new();
        let nonce = generate_nonce();
        let sig = {
            let message = format!("{nonce}{}", self.body);
            cred.signature(&message)
        };
        out.insert(ACCESS_KEY, cred.api_key.try_into()?);
        out.insert(ACCESS_NONCE, nonce.try_into()?);
        out.insert(ACCESS_SIGNATURE, sig.try_into()?);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let cred = Credential {
            api_key: "MY_API_KEY".to_owned(),
            api_secret: "MY_API_SECRET".to_owned(),
        };
        let get = GetAuth {
            path: "/v1/a/b/c".to_owned(),
            params: "?a=1&b=2".to_owned(),
        };
        let h = get.create(cred.clone())?;
        dbg!(h);

        let post = PostAuth {
            body: "{a:1,b:[2,3]}".to_owned(),
        };
        let h = post.create(cred)?;
        dbg!(h);

        Ok(())
    }
}
