extern crate sha1;
extern crate deflate;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::io;

fn hash(s: &String) -> String {
    let mut m = sha1::Sha1::new();
    m.update(s.as_bytes());
    m.digest().to_string()
}

fn compress(s: &String) -> Vec<u8> {
    deflate::deflate_bytes_zlib(s.as_bytes()) 
}

pub fn write_object(content: &str) -> Result<(), io::Error> {
    let header = format!("blob {}\0", content.len());
    let store = header + content;

    let hash = hash(&store);
    let bytes = compress(&store);

    let path = format!("{}/{}", ".git/objects", &hash[0..2]);
    fs::create_dir_all(&path)?;

    let fname = format!("{}/{}", path, &hash[2..40]);
    let mut f = File::create(fname)?;
    f.write_all(&bytes)?;

    Ok(())
}
