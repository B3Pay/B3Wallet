use ic_stable_structures::{writer::Writer, Memory};
use std::borrow::BorrowMut;

use super::types::DefaultVM;

mod test;

pub struct BackupPartition(DefaultVM);

impl BackupPartition {
    pub fn init(default_vm: DefaultVM) -> Self {
        Self(default_vm)
    }

    pub fn backup(&self) -> &DefaultVM {
        &self.0
    }

    pub fn backup_mut(&mut self) -> &mut DefaultVM {
        &mut self.0
    }

    pub fn len(&self) -> u64 {
        self.0.size()
    }

    pub fn get_backup(&self) -> Vec<u8> {
        // Read the length of the state bytes.
        let mut state_len_bytes = [0; 4];
        self.0.read(0, &mut state_len_bytes);

        let state_len = u32::from_le_bytes(state_len_bytes);

        // Read the bytes
        let state_bytes = self.read_backup(4, state_len);

        state_bytes
    }

    pub fn read_backup(&self, offset: u64, len: u32) -> Vec<u8> {
        let mut state_bytes = vec![0u8; len as usize];

        self.0.read(offset, &mut state_bytes);

        state_bytes
    }

    pub fn set_backup(&mut self, state_bytes: Vec<u8>) {
        let len = state_bytes.len() as u32;

        let memory = self.0.borrow_mut();

        let mut writer = Writer::new(memory, 0);
        writer.write(&len.to_le_bytes()).unwrap();
        writer.write(&state_bytes).unwrap()
    }

    pub fn write_backup(&mut self, offset: u64, state_bytes: &[u8]) {
        self.0.write(offset, state_bytes)
    }

    pub fn clear_backup(&mut self) {
        self.set_backup(vec![])
    }
}
