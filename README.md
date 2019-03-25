# Just thinking out loud...

bare minimum:
write k/v docs, collect and link them by id
keys = bytes
values = bytes
ids = integer

# ID gen options
- BYO
- Assigned
  - break id space into chunks, assign chunks to nodes. means max nodes are a hard limit but that's probably fine. make it setup config (of the collection?).
  - leader election and replication for id assignment. slower...

# Keyspaces
- docs:
  - key
  - value
- collections:
  - entity
  - id
  - key
- links:
  - key
  - to_entity
  - ids

# Operations
- write:
  - put docs:key/value
- assign(key,entity,id):
  - put collections:entity/id/key
- link:
  - merge links:key/to_entity/[ids+id]
- get(key):
  - get docs:key/value
- is_linked_to(from_key,to_entity,to_id):
  - get links:key/to_entity/ids.contains(to_id)
- get_linked_ids(from_key):
  - get links:key/to_entity/ids
- get_linked_keys(from_key):
  - get links:key/to_entity/ids
  - get-each collections:to_entity/[id]/key
- is_linked_to(from_key,to_key):
  - get links:key/to_entity/ids
  - get-until collections:to_entity/[id]/key == to_key

# Other ideas
What if instead of using collections and ids we used multiple bitmaps
One per 8 byte segment and then answer membership questions via iterating checking all bitmaps
Would require a length indicator to avoid collisions with longer keys...
- 56 bits and 8 for key length? Maximum key size of 255... which would be 37 bitmaps
- 55 bits and 9 for key length? Maximum key size of 512... which would be 75 bitmaps
- 54 bits and 10 for key length? Maximum key size of 1024... which would be 151 bitmaps
- 48 bits and 16 for key length? Maximum key size of 65535... which would be 10923 bitmaps
Probably susceptible to horrible regressive bitmap behaviour... unless keys are mostly the same length and have high adjacency.

What if we used a tree for this? Solves the sparse bitmap problem and the length indicator...
How to persist? rpds? serde + Vec<u8>?
- bincode - 10 bytes per unique byte added
- msgpack - 4 bytes per unique byte added
- custom bitmap hashkey serializer? worst case 2 bytes per unique byte added
  - [1 magic byte|32 key bytes|32 match bytes|...children_in_ascending_order]
  - switch to bitmap when you hit 31 length prefixed entries?
  - magic byte describes both key bytes and match bytes (list or bitmap)