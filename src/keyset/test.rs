use super::*;

#[test]
pub fn can_serialize_then_deserialize_a_single_byte_keyset() {
    let mut original = KeySet::new();
    original.add(&vec![68]);
    let serialized = original.serialize();
    let deserialized = KeySet::deserialize(&serialized).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
pub fn can_serialize_then_deserialize_a_multi_entry_keyset() {
    let mut original = KeySet::new();
    original.add(&vec![78, 88]);
    original.add(&vec![88]);
    original.add(&vec![88, 120, 0, 45]);
    original.add(&vec![88, 120, 109, 45]);
    let serialized = original.serialize();
    let deserialized = KeySet::deserialize(&serialized).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
pub fn serialized_size_grows_by_2_bytes_per_unique_byte_before_32_at_same_node() {
    let mut original = KeySet::new();
    original.add(&vec![88, 120, 0]);
    original.add(&vec![88, 120, 1]);
    original.add(&vec![88, 120, 2]);
    original.add(&vec![88, 120, 3]);
    original.add(&vec![88, 120, 4]);
    original.add(&vec![88, 120, 5]);
    original.add(&vec![88, 120, 6]);
    original.add(&vec![88, 120, 7]);
    original.add(&vec![88, 120, 8]);
    original.add(&vec![88, 120, 9]);
    original.add(&vec![88, 120, 10]);
    original.add(&vec![88, 120, 11]);
    original.add(&vec![88, 120, 12]);
    original.add(&vec![88, 120, 13]);
    original.add(&vec![88, 120, 14]);
    original.add(&vec![88, 120, 15]);
    original.add(&vec![88, 120, 16]);
    original.add(&vec![88, 120, 17]);
    original.add(&vec![88, 120, 18]);
    original.add(&vec![88, 120, 19]);
    
    original.add(&vec![88, 120, 20]);
    original.add(&vec![88, 120, 21]);

    let serialized1 = original.serialize();
    
    original.add(&vec![88, 120, 22]);
    original.add(&vec![88, 120, 23]);
    original.add(&vec![88, 120, 24]);
    original.add(&vec![88, 120, 25]);
    original.add(&vec![88, 120, 26]);
    original.add(&vec![88, 120, 27]);
    original.add(&vec![88, 120, 28]);
    original.add(&vec![88, 120, 29]);
        
    let serialized2 = original.serialize();
    
    let deserialized1 = KeySet::deserialize(&serialized1).unwrap();
    let deserialized2 = KeySet::deserialize(&serialized2).unwrap();
    
    assert_ne!(original, deserialized1);
    assert_eq!(original, deserialized2);
    
    assert_eq!(serialized1.len() + 16, serialized2.len());
}

#[test]
pub fn serialized_size_grows_by_1_byte_per_unique_byte_after_32_at_same_node() {
    let mut original = KeySet::new();
    original.add(&vec![88, 120, 0]);
    original.add(&vec![88, 120, 1]);
    original.add(&vec![88, 120, 2]);
    original.add(&vec![88, 120, 3]);
    original.add(&vec![88, 120, 4]);
    original.add(&vec![88, 120, 5]);
    original.add(&vec![88, 120, 6]);
    original.add(&vec![88, 120, 7]);
    original.add(&vec![88, 120, 8]);
    original.add(&vec![88, 120, 9]);
    original.add(&vec![88, 120, 10]);
    original.add(&vec![88, 120, 11]);
    original.add(&vec![88, 120, 12]);
    original.add(&vec![88, 120, 13]);
    original.add(&vec![88, 120, 14]);
    original.add(&vec![88, 120, 15]);
    original.add(&vec![88, 120, 16]);
    original.add(&vec![88, 120, 17]);
    original.add(&vec![88, 120, 18]);
    original.add(&vec![88, 120, 19]);
    
    original.add(&vec![88, 120, 20]);
    original.add(&vec![88, 120, 21]);
    original.add(&vec![88, 120, 22]);
    original.add(&vec![88, 120, 23]);
    original.add(&vec![88, 120, 24]);
    original.add(&vec![88, 120, 25]);
    original.add(&vec![88, 120, 26]);
    original.add(&vec![88, 120, 27]);
    original.add(&vec![88, 120, 28]);
    original.add(&vec![88, 120, 29]);
    
    original.add(&vec![88, 120, 30]);
    original.add(&vec![88, 120, 31]);
    
    let serialized1 = original.serialize();
    
    original.add(&vec![88, 120, 32]);
    original.add(&vec![88, 120, 33]);
    original.add(&vec![88, 120, 34]);
    original.add(&vec![88, 120, 35]);
    original.add(&vec![88, 120, 36]);
    original.add(&vec![88, 120, 37]);
    original.add(&vec![88, 120, 38]);
    original.add(&vec![88, 120, 39]);
    
    let serialized2 = original.serialize();
    
    let deserialized1 = KeySet::deserialize(&serialized1).unwrap();
    let deserialized2 = KeySet::deserialize(&serialized2).unwrap();
    
    assert_ne!(original, deserialized1);
    assert_eq!(original, deserialized2);
    
    assert_eq!(serialized1.len() + 8, serialized2.len());
}