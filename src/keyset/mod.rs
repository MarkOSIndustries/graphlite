use std::collections::HashMap;
// use std::collections::bit_vec::BitVec;
use bitvec::*;

// TODO: Implement a read-only version which does not allocate any RAM beyond the serialized buffer

const MAGIC_BYTE_LIST_LENGTH_MASK: u8   = 0b0001_1111;
const MAGIC_BYTE_CHILDREN_IS_BITMAP: u8 = 0b0010_0000;
const MAGIC_BYTE_IS_MATCH: u8           = 0b1000_0000;

#[derive(PartialEq,Debug)]
pub struct KeySet {
    children: HashMap<u8,Self>,
    is_match: bool,
}

impl KeySet {
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
            0 => magic_byte,
            len if len < 32 => {
                child_exists_bytes = child_exists.iter().enumerate()
                    .filter(|(_byte, exists)| *exists)
                    .map(|(byte, _exists)| byte as u8)
                    .collect();
                magic_byte | (len as u8 & MAGIC_BYTE_LIST_LENGTH_MASK)
            },
            _ => {
                child_exists_bytes = child_exists.as_slice().to_vec();
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
        match buf.split_first() {
            None => Err("Not enough bytes".to_string()),
            Some((magic_byte, mut buf)) => {
                let mut child_exists = bitvec![BigEndian, u8; 0; 256];
                let mut next = Self::new();
                next.is_match = MAGIC_BYTE_IS_MATCH == magic_byte & MAGIC_BYTE_IS_MATCH;
                if MAGIC_BYTE_CHILDREN_IS_BITMAP == magic_byte & MAGIC_BYTE_CHILDREN_IS_BITMAP {
                    if buf.len() < 32 {
                        return Err("Not enough bytes".to_string())
                    }
                    child_exists = buf[0..32].into();
                    buf = &buf[32..];
                } else /* It's a list */ {
                    let length = magic_byte & MAGIC_BYTE_LIST_LENGTH_MASK;
                    for _ in 0..length {
                        if let Some((byte, remaining_buf)) = buf.split_first() {
                            child_exists.set(*byte as usize, true);
                            buf = remaining_buf;
                        } else {
                            return Err("Not enough bytes".to_string());
                        }
                    }
                }
                
                for (byte, _exists) in child_exists.iter().enumerate().filter(|(_byte, exists)| *exists) {
                    match Self::deserialize_next(buf) {
                        Err(err) => return Err(err),
                        Ok((child, remaining_buf)) => {
                            next.children.insert(byte as u8, child);
                            buf = remaining_buf;
                        },
                    }
                }
                Ok((next, buf))
            },
        }
    }
}

#[cfg(test)]
mod test;