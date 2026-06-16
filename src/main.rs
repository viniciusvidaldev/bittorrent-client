use std::env;

use anyhow::{Context, Result, anyhow, bail};

#[derive(Debug)]
enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
}

fn decode_bencoded_value(encoded_value: &str) -> Result<BencodeValue> {
    match encoded_value.chars().next() {
        Some('i') => {
            let end = encoded_value.find('e').context("missing 'e' in interger")?;
            let raw = &encoded_value[1..end];
            if raw == "-0" {
                bail!("invalid integer: -0 is not allowed");
            }
            let n: i64 = raw.parse().map_err(|e| anyhow!("invalid integer: {e}"))?;
            Ok(BencodeValue::Integer(n))
        }

        Some('0'..='9') => {
            let (len_str, rest) = encoded_value
                .split_once(':')
                .context("missing ':' in string")?;
            let len: usize = len_str
                .parse()
                .map_err(|e| anyhow!("invalid length: {e}"))?;
            Ok(BencodeValue::String(rest[..len].as_bytes().to_vec()))
        }
        _ => bail!("unsupported bencode value: {}", encoded_value),
    }
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let command = args.next();
    match command.as_deref() {
        Some("decode") => {
            let encoded = args.next().context("missing argument: encoded value")?;
            let value = decode_bencoded_value(&encoded)?;
            match value {
                BencodeValue::Integer(i) => println!("{i}"),
                BencodeValue::String(s) => println!("{}", String::from_utf8_lossy(&s)),
            }
        }
        Some(cmd) => println!("unknown command: {cmd}"),
        None => println!("no command provided"),
    }
    Ok(())
}
