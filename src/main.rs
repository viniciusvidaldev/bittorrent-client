use std::{env, fmt};

use anyhow::{Context, Result, anyhow, bail};

#[derive(Debug)]
enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
}

impl fmt::Display for BencodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BencodeValue::Integer(i) => write!(f, "{i}"),
            BencodeValue::String(s) => write!(f, "{}", String::from_utf8_lossy(s)),
            BencodeValue::List(l) => {
                writeln!(f, "[")?;
                for (i, val) in l.iter().enumerate() {
                    if i + 1 == l.len() {
                        writeln!(f, "  {val}")?;
                    } else {
                        writeln!(f, "  {val},")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

fn decode_bencoded_value(encoded_value: &str) -> Result<(BencodeValue, &str)> {
    match encoded_value.as_bytes().first() {
        Some(b'i') => {
            let rest = &encoded_value[1..]; // skip leading "i"
            let end = rest.find('e').context("missing 'e' in integer")?;
            let raw = &rest[..end];
            if raw == "-0" {
                bail!("invalid integer: -0 is not allowed");
            }
            let n: i64 = raw.parse().map_err(|e| anyhow!("invalid integer: {e}"))?;
            Ok((BencodeValue::Integer(n), &rest[end + 1..])) // [end + 1..] to skip terminating "e"
        }
        Some(b'l') => {
            let mut rest = &encoded_value[1..]; // skip leading "l"
            let mut values = Vec::new();
            while !rest.starts_with('e') {
                let (val, remaining) = decode_bencoded_value(rest)?;
                values.push(val);
                rest = remaining;
            }
            Ok((BencodeValue::List(values), &rest[1..])) // [1..] to skip terminating "e"
        }
        Some(b'0'..=b'9') => {
            let (len_str, rest) = encoded_value
                .split_once(':')
                .context("missing ':' in string")?;
            let len: usize = len_str
                .parse()
                .map_err(|e| anyhow!("invalid length: {e}"))?;
            Ok((
                BencodeValue::String(rest[..len].as_bytes().to_vec()),
                &rest[len..],
            ))
        }
        None => bail!("unexpected end of input or empty string"),
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
            let (value, _) = decode_bencoded_value(&encoded)?;
            println!("{value}");
        }
        Some(cmd) => println!("unknown command: {cmd}"),
        None => println!("no command provided"),
    }
    Ok(())
}
