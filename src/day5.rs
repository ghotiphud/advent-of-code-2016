extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;


pub fn main() {
    let key = "cxdnnyjw".as_bytes();

    let mut password = [0;8];
    let mut password2 = [0;8];

    let mut mask = [false;8];

    for (index, (a, b)) in hashes(&key).enumerate() {
        if index < 8 {
            password[index] = a;
        }
        
        let pos = a as usize;
        let byte = b;
        if pos < 8 && !mask[pos] {
            password2[pos] = byte;
            mask[pos] = true;

            if mask.iter().all(|&v| v) {
                break;
            }
        }
    }

    for byte in &password {
        print!("{:X}", byte);
    }
    println!("");


    for byte in &password2 {
        print!("{:X}", byte);
    }
    println!("");
}

fn hashes<'a>(key: &'a [u8]) -> impl Iterator<Item=(u8,u8)> + 'a {
    let mut hasher = Md5::new();

    (0..u64::max_value()).map(move |i| {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0;16];
        hasher.result(&mut output);

        hasher.reset();

        let first_five = output[0] as i32 + 
                         output[1] as i32 + 
                         (output[2] >> 4) as i32;
        
        if first_five == 0 {
            Some((output[2] & 0x0f, output[3] >> 4))
        } else {
            None
        }
    })
    .filter_map(|x| x)
}