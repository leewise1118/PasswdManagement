use anyhow::{bail, Error, Result};
use base64::{engine::general_purpose, Engine};
use config::CRYPTO;
use hash::merhash::mersenne_hash;
use log::{self, info};

pub fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    if length < 6 {
        bail!("密码长度不能小于6");
    }

    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };

    let mut merhash = mersenne_hash(seed).pow(p);
    info!("merhash = {}", merhash);

    let mut passwd = String::new();
    let crypto_len = CRYPTO.len();
    while merhash > 9 {
        // 将哈希之后的值作为索引，求得密码
        let loc = merhash % crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while getting char!");
        passwd.push(nthc);
        merhash /= crypto_len;
    }
    info!("after hash, passwd = {}", passwd);

    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }
    passwd = general_purpose::STANDARD.encode(passwd);
    info!("after encode, passwd = {}", passwd);
    passwd = passwd.replace("+", "*").replace("/", "*");

    let interval = passwd.clone();
    while passwd.len() < length {
        passwd += &interval;
    }
    info!("final, passwd = {}", passwd);
    // 返回前length个字符作为密码
    Ok(format!("{}: {}", seed, &passwd[..length]))
}
