use std::collections::HashMap;
// use std::collections::bit_vec::BitVec;
use bitvec::*;

const MAGIC_BYTE_CHILDREN_IS_BITMAP: u8 = 0b1000_0000;
const MAGIC_BYTE_CHILDREN_IS_LIST: u8   = 0b0100_0000;
const MAGIC_BYTE_CHILDREN_IS_EMPTY: u8  = 0b0010_0000;
const MAGIC_BYTE_IS_MATCH: u8           = 0b0000_0001;

#[derive(Debug)]
pub struct BinaryKeySet {
    children: HashMap<u8,Self>,
    is_match: bool,
}

impl BinaryKeySet {
    pub fn new() -> Self {
        Self { is_match: false, children: HashMap::new() }
    }
    
    pub fn contains(&self, key: &[u8]) -> bool {
        let mut node = self;
        let mut key = &key[..];
        loop {
            match key.split_first() {
                None => return node.is_match,
                Some((key_first_byte, key_remaining_bytes)) => match node.children.get(key_first_byte) {
                    None => return false,
                    Some(child) => {
                        node = child;
                        key = key_remaining_bytes;
                    },
                },
            }
        }
    }

    pub fn add(&mut self, key: &[u8]) {
        if !key.is_empty() {
            let mut node = self;
            let mut key = &key[..];
            loop {
                match key.split_first() {
                    None => {
                        node.is_match = true;
                        break;
                    },
                    Some((first_byte, remaining_bytes)) => {
                        node = node.children
                            .entry(*first_byte)
                            .or_insert_with(|| Self::new());
                        key = remaining_bytes;
                    },
                }
            }
        }
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        let mut child_exists = bitvec![BigEndian, u8; 0; 256];
        let mut child_data: Vec<u8> = vec![];
        
        // TODO: stop using hashmaps in RAM - use bitmaps+Vec<Option<Self>>
        // Iterate all possible values to make sure we're in order
        for byte in 0..=255 {
            match self.children.get(&byte) {
                None => continue,
                Some(child) => {
                    child_exists.set(byte as usize, true);
                    child_data.append(&mut child.serialize());
                },
            }
        }

        let mut child_exists_bytes: Vec<u8> = vec![];
        let magic_byte: u8 = if self.is_match {MAGIC_BYTE_IS_MATCH} else {0};
        let magic_byte = match self.children.len() {
            0 => magic_byte | MAGIC_BYTE_CHILDREN_IS_EMPTY,
            len if len < 32 => {
                let mut child_exists_as_vec: Vec<u8> = child_exists.iter().enumerate().filter(|(_byte, exists)| *exists).map(|(byte, _exists)| byte as u8).collect();
                child_exists_bytes.push(len as u8);
                child_exists_bytes.append(&mut child_exists_as_vec);
                magic_byte | MAGIC_BYTE_CHILDREN_IS_LIST
            },
            _ => {
                child_exists_bytes.append(&mut child_exists.as_mut_slice().to_vec());
                magic_byte | MAGIC_BYTE_CHILDREN_IS_BITMAP
            },
        };
        
        [
            &[magic_byte] as &[u8],
            &child_exists_bytes[..],
            &child_data[..],
        ].concat().to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, String> {
        match Self::deserialize_next(buf) {
            Ok((x, remaining_bytes)) => {
                if remaining_bytes.is_empty() {
                    Ok(x)
                } else {
                    Err(format!("Too many bytes: {:?}", remaining_bytes))
                }
            }
            Err(err) => Err(err)
        }
    }

    fn deserialize_next(buf: &[u8]) -> Result<(Self, &[u8]), String> {
        if let Some((magic_byte, buf)) = buf.split_first() {
            let mut owned_children = bitvec![BigEndian, u8; 0; 256];
            let mut next = BinaryKeySet::new();
            next.is_match = MAGIC_BYTE_IS_MATCH == magic_byte & MAGIC_BYTE_IS_MATCH;
            match if MAGIC_BYTE_CHILDREN_IS_LIST == magic_byte & MAGIC_BYTE_CHILDREN_IS_LIST {
                if let Some((length, mut buf)) = buf.split_first() {
                    let mut error = None;
                    for _ in 0..*length {
                        if let Some((byte, remaining_buf)) = buf.split_first() {
                            owned_children.set(*byte as usize, true);
                            buf = remaining_buf;
                        } else {
                            error = Some(Err("Not enough bytes".to_string()));
                            break;
                        }
                    }
                    match error {
                        Some(err) => err,
                        None => Ok((&owned_children[..], buf)),
                    }
                } else {
                    Err("Not enough bytes".to_string())
                }
            } else if MAGIC_BYTE_CHILDREN_IS_BITMAP == magic_byte & MAGIC_BYTE_CHILDREN_IS_BITMAP {
                if buf.len() < 32 {
                    Err("Not enough bytes".to_string())
                } else {
                    Ok((buf[0..32].into(), &buf[32..]))
                }
            } else if MAGIC_BYTE_CHILDREN_IS_EMPTY == magic_byte & MAGIC_BYTE_CHILDREN_IS_EMPTY {
                Ok((&owned_children[..], buf))
            } else {
                Err(format!("Invalid magic byte {}", magic_byte))
            } {
                Err(err) => return Err(err),
                Ok((child_exists, mut buf)) => {
                    for (byte, _exists) in child_exists.iter().enumerate().filter(|(_byte, exists)| *exists) {
                        match Self::deserialize_next(buf) {
                            Ok((child, remaining_buf)) => {
                                next.children.insert(byte as u8, child);
                                buf = remaining_buf;
                            },
                            Err(err) => return Err(err),
                        }
                    }
                    return Ok((next, buf));
                }
            }
        }

        return Err("Not enough bytes".to_string());
    }
}
