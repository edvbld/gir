extern crate ring;
extern crate flate2;
extern crate rustc_serialize;

use ring::digest;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;
use std::fs;
use std::fs::File;
use rustc_serialize::hex::ToHex;

fn main() {
    let content = "what is up, doc?\n";
    let header = format!("blob {}\0", content.len());
    let store = header + content;
    let store_bytes = store.as_bytes();
    let digest = digest::digest(&digest::SHA1, store_bytes);
    let foo = digest.as_ref().to_hex();
    let sha1 = foo.as_str();
    println!("{}", sha1);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write(store_bytes).unwrap();
    let bytes = encoder.finish().unwrap();
    println!("{}", bytes.to_hex());

    let path = format!("{}/{}", ".git/objects", &sha1[0..2]);
    fs::create_dir_all(&path).unwrap();

    let fname = format!("{}/{}", path, &sha1[2..40]);
    let mut f = File::create(fname).unwrap();
    f.write_all(&bytes).unwrap();
}
