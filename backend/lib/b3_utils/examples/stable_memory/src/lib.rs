use b3_utils::logs::{export_log, export_log_messages_page, LogEntry};
use b3_utils::memory::timer::{DefaultTaskTimer, TaskTimerEntry};
use b3_utils::memory::types::{
    Bound, DefaultVMHeap, DefaultVMLog, DefaultVMMap, DefaultVMVec, PartitionDetail, Storable,
};
use b3_utils::memory::{
    init_stable_mem_refcell, with_backup_mem, with_backup_mem_mut, with_stable_mem,
};
use b3_utils::{log, require, require_log, NanoTimeStamp, Subaccount};
use candid::CandidType;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;

use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::io::Cursor;

const MAX_OPERATION_SIZE: u32 = 200;

thread_local! {
    static TASK_TIMER: RefCell<DefaultTaskTimer<Task>> = init_stable_mem_refcell("timer", 1).unwrap();

    static MAP: RefCell<DefaultVMMap<u64, User>> = init_stable_mem_refcell("map", 10).unwrap();
    static HEAP: RefCell<DefaultVMHeap<u64>> = init_stable_mem_refcell("heap", 11).unwrap();
    static USERS: RefCell<DefaultVMMap<u64, User>> = init_stable_mem_refcell("users", 12).unwrap();
    static SUBACCOUNTS: RefCell<DefaultVMMap<Subaccount, User>> = init_stable_mem_refcell("subaccounts", 13).unwrap();
    static STABLE_LOG: RefCell<DefaultVMLog<Subaccount>> = init_stable_mem_refcell("logs", 14).unwrap();
    static VEC: RefCell<DefaultVMVec<ProcessedOperation>> = init_stable_mem_refcell("ledger", 15).unwrap();

    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
enum Task {
    SumAndLog(u64, u64),
    SumAndLogSub(u64, u64),
    SumAndLogSubWithRequire(u64, u64),
}

impl Storable for Task {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: MAX_OPERATION_SIZE,
    };
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

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: MAX_OPERATION_SIZE,
    };
}

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    Success,
    Fail,
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

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: MAX_OPERATION_SIZE,
    };
}

#[init]
fn init() {
    log!("init: {}", ic_cdk::api::id());
}

#[query]
fn get_operation(id: u64) -> Option<ProcessedOperation> {
    VEC.with(|p| p.borrow().get(id))
}

#[update]
fn append_operation(operation: ProcessedOperation) -> Result<(), String> {
    VEC.with(|p| p.borrow_mut().push(&operation))
        .map_err(|e| e.to_string())
}

#[query]
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
fn add_user(user: User) -> Option<User> {
    USERS.with(|s| {
        let mut state = s.borrow_mut();

        state.insert(user.id, user)
    })
}

#[update]
fn add_user_with_operation(user: User, operation: ProcessedOperation) {
    USERS.with(|s| {
        let mut state = s.borrow_mut();

        state.insert(user.id, user)
    });
    VEC.with(|p| p.borrow_mut().push(&operation)).unwrap();
}

#[update]
fn update_state(state: State) {
    STATE.with(|s| {
        *s.borrow_mut() = state;
    })
}

#[query]
fn get_state() -> State {
    STATE.with(|s| s.borrow().clone())
}

#[query]
fn get_users() -> Vec<User> {
    USERS.with(|s| {
        let state = s.borrow();

        state.iter().map(|(_, user)| user.clone()).collect()
    })
}

#[query]
fn get_user_len() -> u64 {
    USERS.with(|s| {
        let state = s.borrow();

        state.len()
    })
}

#[query]
fn get_timers() -> Vec<TaskTimerEntry<Task>> {
    TASK_TIMER.with(|s| {
        let state = s.borrow();

        state.get_timers()
    })
}

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

#[query]
fn print_log_entries_page(page: usize, page_size: Option<usize>) -> Vec<String> {
    export_log_messages_page(page, page_size)
}

#[update]
fn sum_and_log(x: u64, y: u64) -> u64 {
    let result = x.saturating_add(y);

    log!("sum_and_log: {} + {} = {}", x, y, result);

    STABLE_LOG
        .with(|s| s.borrow_mut().append(&Subaccount::from([0; 32])))
        .unwrap();

    result
}

#[update]
fn sum_and_log_sub(x: u64, y: u64) -> Result<u64, String> {
    require!(x >= y, "y({}) must be less than x({})", y, x);

    let result = x.saturating_sub(y);

    log!("sum_and_log_sub: {} - {} = {}", x, y, result);

    Ok(result)
}

pub fn sub(x: u64, y: u64) -> Result<u64, String> {
    if x < y {
        return Err(format!("y({}) must be less than x({})", y, x));
    }

    Ok(x.saturating_sub(y))
}

#[update]
fn sum_and_log_sub_with_require(x: u64, y: u64) -> Result<u64, String> {
    require_log!(x >= y, "y({}) must be less than x({})", y, x);

    let result = x.saturating_sub(y);

    log!("sum_and_log_sub_with_require: {} - {} = {}", x, y, result);

    Ok(result)
}

#[query]
fn get_partition_details() -> Vec<PartitionDetail> {
    with_stable_mem(|pm| pm.partition_details())
}

#[query]
fn get_backup_memory() -> Vec<u8> {
    with_backup_mem(|bp| bp.get_backup())
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

    with_backup_mem_mut(|backup| backup.set_backup(state_bytes));
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade() {
    log!("post_upgrade: {}", ic_cdk::api::id());

    let state_bytes = with_backup_mem(|bp| bp.get_backup());

    log!("state_bytes: {}", state_bytes.len());

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| *s.borrow_mut() = state);

    reschedule();
}

#[update]
fn schedule_task(after_sec: u64, task: Task) {
    let time = NanoTimeStamp::now().add_secs(after_sec);

    let timer = TaskTimerEntry { task, time };

    TASK_TIMER
        .with(|tt| {
            let mut tt = tt.borrow_mut();

            tt.push_timer(&timer)
        })
        .unwrap();

    reschedule();
}

#[export_name = "canister_global_timer"]
fn global_timer() {
    while let Some(task_time) = TASK_TIMER.with(|tt| {
        let tt = tt.borrow();

        tt.peek_timer()
    }) {
        if task_time.time.in_future() {
            reschedule();
            return;
        }
        TASK_TIMER.with(|tt| {
            let mut tt = tt.borrow_mut();

            tt.pop_timer()
        });

        execute_task(task_time);
        reschedule();
    }
}

fn execute_task(timer: TaskTimerEntry<Task>) {
    log!("execute_task: {:?}", timer.task);
    log!("execute_task in : {}", timer.time);

    add_user(User {
        id: timer.time.clone().into(),
        name: format!("{:?}", timer.task),
        email: format!("{}@test.com", timer.time.to_secs()),
        new_field: None,
        created_at: NanoTimeStamp::now(),
    });
}

fn reschedule() {
    if let Some(task_time) = TASK_TIMER.with(|tt| {
        let tt = tt.borrow();

        tt.peek_timer()
    }) {
        unsafe {
            ic0::global_timer_set(task_time.time.into());
        }
    }
}

ic_cdk::export_candid!();
