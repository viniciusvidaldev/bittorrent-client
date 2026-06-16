use std::{collections::HashMap, env, fmt};

use anyhow::{Context, Result, anyhow, bail};

#[derive(PartialEq)]
enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
    Dict(HashMap<Vec<u8>, BencodeValue>),
}

impl fmt::Debug for BencodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BencodeValue::String(s) => write!(f, "{:?}", String::from_utf8_lossy(s)),
            BencodeValue::Integer(i) => write!(f, "{i}"),
            BencodeValue::List(l) => f.debug_list().entries(l).finish(),
            BencodeValue::Dict(d) => f
                .debug_map()
                .entries(d.iter().map(|(k, v)| (String::from_utf8_lossy(k), v)))
                .finish(),
        }
    }
}

fn decode_bencoded_value(encoded_value: &str) -> Result<(BencodeValue, &str)> {
    match encoded_value.chars().next() {
        Some('i') => {
            let rest = &encoded_value[1..]; // skip leading "i"
            let end = rest.find('e').context("missing 'e' in integer")?;
            let raw = &rest[..end];
            if raw == "-0" {
                bail!("invalid integer: -0 is not allowed");
            }
            let n: i64 = raw.parse().map_err(|e| anyhow!("invalid integer: {e}"))?;
            Ok((BencodeValue::Integer(n), &rest[end + 1..])) // [end + 1..] to skip terminating "e"
        }
        Some('l') => {
            let mut rest = &encoded_value[1..]; // skip leading "l"
            let mut values = Vec::new();
            while !rest.starts_with('e') {
                let (val, remaining) = decode_bencoded_value(rest)?;
                values.push(val);
                rest = remaining;
            }
            Ok((BencodeValue::List(values), &rest[1..])) // [1..] to skip terminating "e"
        }
        Some('d') => {
            let mut map = HashMap::new();
            let mut rest = &encoded_value[1..]; // skip leading "d"
            while !rest.starts_with('e') {
                let (key, remaining) = decode_bencoded_value(rest)?;
                rest = remaining;
                let BencodeValue::String(key) = key else {
                    bail!("dict keys must be strings, got {:?}", key);
                };

                let (val, remaining) = decode_bencoded_value(rest)?;
                rest = remaining;
                map.insert(key, val);
            }

            Ok((BencodeValue::Dict(map), &rest[1..])) // [1..] to skip terminating "e"
        }
        Some('0'..='9') => {
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
            println!("{value:?}");
        }
        Some(cmd) => println!("unknown command: {cmd}"),
        None => println!("no command provided"),
    }
    Ok(())
}
