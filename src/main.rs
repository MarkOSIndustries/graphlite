mod keyset;

use sled::Db;
use std::path;

fn main() {
    let mut bktn1 = keyset::KeySet::new();
    bktn1.add(&vec![68]);
    let bktn1 = bktn1;

    let mut bktn2 = keyset::KeySet::new();
    bktn2.add(&vec![78, 88]);
    bktn2.add(&vec![88]);
    bktn2.add(&vec![88, 120, 99, 45]);
    bktn2.add(&vec![88, 120, 109, 45]);
    let bktn2 = bktn2;

    let k: Vec<u8> = vec![127];
    let v1: Vec<u8> = bktn1.serialize();
    let v2: Vec<u8> = bktn2.serialize();

    let path = path::Path::new("c:\\tmp\\eventdex");
    if let Ok(tree) = Db::start_default(path) {
        
        // set and get
        if let Err(err) = tree.set(&k, v1.clone()) {
            println!("Couldn't set {}", err);
        }
        match tree.get(&k) {
            Ok(Some(v)) => {
                println!("Got {} [{}] {:?}", k[0], v.len(), &v);
                println!("Got {} [{}] {:?} {:?}", k[0], v.len(), &v, &keyset::KeySet::deserialize(&v));
            },
            Ok(None) => println!("Got {} None", k[0]),
            Err(err) => println!("Couldn't get {}", err),
        }
        
        // compare and swap
        if let Err(err) = tree.cas(&k, Some(&v1), Some(v2)) {
            println!("Couldn't compare and swap {}", err);
        }
        
        // scan forward
        let iter = tree.scan(&k);
        for x in iter {
            if let Ok((kk,v)) = x {
                let reconstructed = keyset::KeySet::deserialize(&v).unwrap();
                println!("Iterated {} [{}] {:?} {:?}", kk[0], v.len(), &v, &reconstructed);
                println!("Queried {} {}", 78u8, reconstructed.contains(&vec![78u8]));
                println!("Queried {} {}", 88u8, reconstructed.contains(&vec![88u8]));
                println!("Queried {} {} {}", 78u8, 88u8, reconstructed.contains(&vec![78u8, 88u8]));
            }
        }

        // // deletion
        if let Err(err) = tree.del(&k) {
            println!("Couldn't delete {}", err);
        }

        // block until all operations are on-disk
        if let Err(err) = tree.flush() {
            println!("Couldn't flush {}", err);
        }
    }
}
