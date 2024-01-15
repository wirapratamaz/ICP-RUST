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

// a rait that must be implement for a struct that is stored in a stable struct
impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait tat mut be define max size of the struct and whether it is fixed size
impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Set up thread local variables
thread_local! {
    // variable holds canister virtual memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // variable holds canisster ID counter, which is used to generate unique IDs for messages
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    // variable holds canister storage, enabling access from anywhere
    static STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// set up MessagePayload
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}

// implement core logic of the canister
// get_message function
#[ic_cdk::query]
fn get_message(id: u64) -> Result<Message, Error> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a message with id={} not found", id),
        }),
    }
}

// add_message function
#[ic_cdk::update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment the counter");
    let message = Message {
        id,
        title: message.title,
        body: message.body,
        attachment_url: message.attachment_url,
        created_at: time(),
        updated_at: None,
    };
    do_insert(&message);
    Some(message)   
}

// update_message function
#[ic_cdk::update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut message) => {
            message.title = payload.title;
            message.body = payload.body;
            message.attachment_url = payload.attachment_url;
            message.updated_at = Some(time());
            do_insert(&message);
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "a message with id={} not found", 
                id
            ),
        }),
    }
}

fn do_insert(message: &Message) {
    STORAGE.with(|s| s.borrow_mut().insert(message.id, message.clone()));
}

// delete_message function
#[ic_cdk::update]
fn delete_message(id: u64) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!(
                "a message with id={} not found", 
                id
            ),
        }),
    }
}

// helper method to get message by id. used in get_message and update_message
fn _get_message(id: &u64) -> Option<Message> {
    STORAGE.with(|s| s.borrow().get(id))
}

#[derive(candid::CandidType, Serialize, Deserialize)]
enum Error {
    NotFound { msg: String },   
}

// generate candid
ic_cdk::export_candid!();