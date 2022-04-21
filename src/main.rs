// Alice
// gkQ5K6EnLRgZkwozG8GiBAEnJyM6FxzbSaSmVhKJ2w8FcK7ih
// 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D
//
// Bob
// gkNW9pAcCHxZrnoVkhLkEQtsLsW5NWTC75cdAdxAMs9LNYCYg
// 0x8EAF04151687736326C9FEA17E25FC5287613693C912909CB226AA4794F26A48

use crate::ss58::Address;
use std::env;

mod ss58;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value = args.get(1).expect("Не передано значение");

    let addr = Address::try_from(value.as_str()).unwrap();

    println!("SS58: {}", addr.to_ss58());
    println!("Hex: {}", addr.to_hex());
    println!("Bytes: {:?}", addr.to_bytes());
}
