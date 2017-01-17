extern crate sha1;
extern crate deflate;

use std::fs;
use std::fs::File;
use std::io::Write;

fn sha1(s: &String) -> String {
    let mut m = sha1::Sha1::new();
    m.update(s.as_bytes());
    m.digest().to_string()
}

fn compress(s: &String) -> Vec<u8> {
    deflate::deflate_bytes_zlib(s.as_bytes()) 
}

fn main() {
    let content = "what is up, doc?\n";
    let header = format!("blob {}\0", content.len());
    let store = header + content;

    let hash = sha1(&store);
    println!("{:?}", hash);

    let bytes = compress(&store);
    println!("{:?}", bytes);

    let path = format!("{}/{}", ".git/objects", &hash[0..2]);
    fs::create_dir_all(&path).unwrap();

    let fname = format!("{}/{}", path, &hash[2..40]);
    let mut f = File::create(fname).unwrap();
    f.write_all(&bytes).unwrap();
}
