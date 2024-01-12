// Import dependencies
#[macro_use]
// serde is framework for serializing and deserializing Rust data structures efficiently and generically
extern crate serde;
// import candid so we can use the Encode and Decode traits
use candid::{Decode, Encode};
// import the IC SDK so we can use the ic_cdk::export::candid macro
use ic_cdk::api::time;
// the library use set of data structures that are stable over time
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
// std is the standard library of Rust
use std::{borrow::Cow, cell::RefCell};

//define the memory manager
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>; // Generate a cell with a u64 unique value 

// define message struct
#[derive(candid::CandidType, Clone, Deserialize, Serialize, Default)]
struct Message {
    id: u64,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}




