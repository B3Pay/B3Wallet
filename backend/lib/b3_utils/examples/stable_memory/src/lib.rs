use b3_utils::log;
use b3_utils::logs::{export_log, export_log_messages_page, LogEntry};
use b3_utils::memory::base::{timer::TimerEntry, with_base_partition, with_base_partition_mut};
use b3_utils::memory::types::{
    BoundedStorable, DefaultVMHeap, DefaultVMMap, DefaultVMVec, PartitionDetail, Storable,
};
use b3_utils::memory::{with_stable_memory, with_stable_memory_mut};
use b3_utils::NanoTimeStamp;
use candid::{candid_method, CandidType};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;

use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;

const MAX_OPERATION_SIZE: u32 = 100;

thread_local! {
    static MAP: RefCell<DefaultVMMap<u64, User>> = RefCell::new(with_stable_memory_mut(|pm| pm.init_btree_map("map", 10).unwrap()));
    static HEAP: RefCell<DefaultVMHeap<u64>> = RefCell::new(with_stable_memory_mut(|pm| pm.init_min_heap("heap", 11).unwrap()));
    static USERS: RefCell<DefaultVMMap<u64, User>> = RefCell::new(with_stable_memory_mut(|pm| pm.init_btree_map("users", 12).unwrap()));
    static VEC: RefCell<DefaultVMVec<ProcessedOperation>> = RefCell::new(with_stable_memory_mut(|pm| pm.init_vec("ledger", 13).unwrap()));

    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
    #[serde(default)]
    new_field: Option<u64>,
    #[serde(default = "default_created_at")]
    created_at: NanoTimeStamp,
}

fn default_created_at() -> NanoTimeStamp {
    NanoTimeStamp::from(0)
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = MAX_OPERATION_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    Success,
    Fail,
}

impl BoundedStorable for ProcessedOperation {
    const MAX_SIZE: u32 = MAX_OPERATION_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct ProcessedOperation {
    timestamp: u64,
    method: String,
    error: Option<String>,
    status: OperationStatus,
    #[serde(default = "default_new_field")]
    another_new_field: NewField,
    #[serde(default)]
    new_field: u64,
}

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct NewField {
    value: u64,
}

#[derive(CandidType, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct State {
    ledger: Vec<u8>,
}

fn default_new_field() -> NewField {
    NewField { value: 0 }
}

impl Storable for ProcessedOperation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

#[init]
#[candid_method(init)]
fn init() {
    log!("init.");
}

#[query]
#[candid_method(query)]
fn get_operation(id: u64) -> Option<ProcessedOperation> {
    VEC.with(|p| p.borrow().get(id))
}

#[update]
#[candid_method(update)]
fn append_operation(operation: ProcessedOperation) -> Result<(), String> {
    VEC.with(|p| p.borrow_mut().push(&operation))
        .map_err(|e| e.to_string())
}

#[query]
#[candid_method(query)]
fn get_operations_range(start: u64, end: u64) -> Vec<ProcessedOperation> {
    let mut operations = Vec::new();
    VEC.with(|p| {
        for id in start..end {
            if let Some(operation) = p.borrow().get(id) {
                operations.push(operation);
            } else {
                break;
            }
        }
    });
    operations
}

#[query]
#[candid_method(query)]
fn get_operations_range_with_state(start: u64, end: u64) -> (Vec<ProcessedOperation>, Vec<User>) {
    let mut operations = Vec::new();
    VEC.with(|p| {
        for id in start..end {
            if let Some(operation) = p.borrow().get(id) {
                operations.push(operation);
            } else {
                break;
            }
        }
    });
    let users = USERS.with(|s| {
        let state = s.borrow();
        state
            .range(start..end)
            .map(|(_, user)| user.clone())
            .collect()
    });
    (operations, users)
}

#[update]
#[candid_method(update)]
fn add_user(user: User) -> Option<User> {
    USERS.with(|s| {
        let mut state = s.borrow_mut();

        state.insert(user.id, user)
    })
}

#[update]
#[candid_method(update)]
fn add_user_with_operation(user: User, operation: ProcessedOperation) {
    USERS.with(|s| {
        let mut state = s.borrow_mut();

        state.insert(user.id, user)
    });
    VEC.with(|p| p.borrow_mut().push(&operation)).unwrap();
}

#[update]
#[candid_method(update)]
fn update_state(state: State) {
    STATE.with(|s| {
        *s.borrow_mut() = state;
    })
}

#[query]
#[candid_method(query)]
fn get_state() -> State {
    STATE.with(|s| s.borrow().clone())
}

#[query]
#[candid_method(query)]
fn get_users() -> Vec<User> {
    USERS.with(|s| {
        let state = s.borrow();

        state.iter().map(|(_, user)| user.clone()).collect()
    })
}

#[query]
#[candid_method(query)]
fn get_user_len() -> u64 {
    USERS.with(|s| {
        let state = s.borrow();

        state.len()
    })
}

#[query]
#[candid_method(query)]
fn get_timers() -> Vec<TimerEntry> {
    with_base_partition(|s| s.get_timer())
}

#[query]
#[candid_method(query)]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

#[query]
#[candid_method(query)]
fn print_log_entries_page(page: u32, page_size: Option<u32>) -> Vec<String> {
    export_log_messages_page(page, page_size)
}

#[update]
#[candid_method(update)]
fn sum_and_log(x: u64, y: u64) -> u64 {
    let result = x.saturating_add(y);

    log!("sum_and_log: {} + {} = {}", x, y, result);

    result
}

#[query]
#[candid_method(query)]
fn get_partition() -> HashMap<String, u8> {
    MAP.with(|_| {});
    VEC.with(|_| {});
    HEAP.with(|_| {});
    USERS.with(|_| {});
    STATE.with(|_| {});

    with_stable_memory(|pm| pm.partitions().clone())
}

#[query]
#[candid_method(query)]
fn get_partition_details() -> Vec<PartitionDetail> {
    let mut details = Vec::new();

    details.push(PartitionDetail {
        name: "map".to_string(),
        len: MAP.with(|m| m.borrow().len()) as u64,
    });

    details.push(PartitionDetail {
        name: "vec".to_string(),
        len: VEC.with(|v| v.borrow().len()) as u64,
    });

    details.push(PartitionDetail {
        name: "users".to_string(),
        len: USERS.with(|u| u.borrow().len()) as u64,
    });

    details.push(PartitionDetail {
        name: "state".to_string(),
        len: STATE.with(|s| s.borrow().ledger.len()) as u64,
    });

    with_base_partition(|bp| {
        bp.details().iter().for_each(|detail| {
            details.push(detail.clone());
        });
    });

    details
}

#[query]
#[candid_method(query)]
fn get_backup_memory() -> Vec<u8> {
    with_base_partition(|core_partition| core_partition.get_backup())
}

// A pre-upgrade hook for serializing the data stored on the heap.
#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    STATE
        .with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.

    with_base_partition_mut(|core_partition| core_partition.set_backup(state_bytes));
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade() {
    log!("post_upgrade: {}", ic_cdk::api::id());

    let state_bytes = with_base_partition(|core_partition| core_partition.get_backup());

    log!("state_bytes: {}", state_bytes.len());

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| *s.borrow_mut() = state);

    reschedule();
}

#[update]
#[candid_method(update)]
fn schedule_task(after_sec: u64, id: u64) {
    let time = NanoTimeStamp::now().add_secs(after_sec);

    let timer = TimerEntry { id, time };

    with_base_partition_mut(|core_partition| core_partition.push_timer(&timer)).unwrap();

    reschedule();
}

#[export_name = "canister_global_timer"]
fn global_timer() {
    while let Some(task_time) = with_base_partition(|core_partition| core_partition.peek_timer()) {
        if task_time.time.in_future() {
            reschedule();
            return;
        }
        with_base_partition_mut(|core_partition| core_partition.pop_timer());

        execute_task(task_time);
        reschedule();
    }
}

fn execute_task(timer: TimerEntry) {
    log!("execute_task: {}", timer.id);
    log!("execute_task in : {}", timer.time);

    add_user(User {
        id: timer.id,
        name: format!("{}", timer.id),
        email: format!("{}@test.com", timer.time),
        new_field: None,
        created_at: NanoTimeStamp::now(),
    });
}

fn reschedule() {
    if let Some(task_time) = with_base_partition(|core_partition| core_partition.peek_timer()) {
        unsafe {
            ic0::global_timer_set(task_time.time.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        let mut file = std::fs::File::create("./candid.did").unwrap();

        export_service!();

        let candid = __export_service();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}
