use std::env;

#[derive(Debug)]
enum BencodeValue {
    String(Vec<u8>),
}

fn decode_bencoded_value(encoded_value: &str) -> BencodeValue {
    match encoded_value.chars().next() {
        Some('0'..='9') => {
            if let Some((len_str, rest)) = encoded_value.split_once(':') {
                if let Ok(len) = len_str.parse::<usize>() {
                    return BencodeValue::String(rest[..len].as_bytes().to_vec());
                }
            }
        }
        _ => {}
    };

    panic!("Unhandled encoded value: {}", encoded_value);
}

fn main() {
    let mut args = env::args();
    args.next(); // skip bin name
    let command = args.next();

    match command.as_deref() {
        Some("decode") => {
            let Some(encoded) = args.next() else {
                println!("missing argument: encoded value");
                return;
            };

            let value = decode_bencoded_value(&encoded);
            if let BencodeValue::String(bytes) = value {
                println!("{}", String::from_utf8_lossy(&bytes));
            }
        }
        Some(cmd) => println!("unknown command: {cmd}"),
        None => println!("no command provided"),
    }
}
