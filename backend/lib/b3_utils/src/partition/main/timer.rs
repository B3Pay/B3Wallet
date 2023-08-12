use std::borrow::Cow;
use std::mem::size_of;

use b3_stable_structures::{BoundedStorable, GrowFailed, Storable};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{partition::DefaultVMHeap, NanoTimeStamp};

use super::MainPartition;

pub type MainTimerType = DefaultVMHeap<TimerEntry>;

#[derive(CandidType, Debug, PartialEq, Eq, Ord, Clone, Serialize, Deserialize)]
pub struct TimerEntry {
    pub time: NanoTimeStamp,
    pub id: u64,
}

impl MainPartition {
    pub fn timer(&self) -> &MainTimerType {
        &self.timer
    }

    pub fn timer_mut(&mut self) -> &mut MainTimerType {
        &mut self.timer
    }

    pub fn get_timer(&self) -> Vec<TimerEntry> {
        self.timer.iter().collect()
    }

    pub fn push_timer(&mut self, timer: &TimerEntry) -> Result<(), GrowFailed> {
        self.timer.push(timer)
    }

    pub fn pop_timer(&mut self) -> Option<TimerEntry> {
        self.timer.pop()
    }

    pub fn clear_timer(&mut self) {
        for _ in 0..self.timer.len() {
            self.timer.pop();
        }
    }

    pub fn peek_timer(&self) -> Option<TimerEntry> {
        self.timer.peek()
    }
}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl BoundedStorable for TimerEntry {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 16;
}

impl Storable for TimerEntry {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![0; size_of::<Self>()];
        let time_bytes = self.time.to_le_bytes();
        let id_bytes = self.id.to_le_bytes();

        bytes[0..8].copy_from_slice(&time_bytes);
        bytes[8..16].copy_from_slice(&id_bytes);

        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let time = NanoTimeStamp::from_le_bytes(bytes[0..8].try_into().unwrap());
        let id = u64::from_le_bytes(bytes[8..16].try_into().unwrap());

        Self { time, id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_entry_to_and_from_bytes() {
        let entry = TimerEntry {
            time: 1234567890.into(),
            id: 9876543210,
        };

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), size_of::<TimerEntry>());

        let entry_from_bytes = TimerEntry::from_bytes(Cow::Borrowed(&bytes));

        assert_eq!(entry, entry_from_bytes);
    }
}
