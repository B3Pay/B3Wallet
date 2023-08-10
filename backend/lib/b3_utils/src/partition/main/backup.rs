use std::borrow::BorrowMut;

use b3_stable_structures::{writer::Writer, Memory};

use crate::partition::DefaultVM;

use super::MainPartition;

pub type MainBackupType = DefaultVM;

impl MainPartition {
    pub fn backup(&self) -> &DefaultVM {
        &self.backup
    }

    pub fn backup_mut(&mut self) -> &mut DefaultVM {
        &mut self.backup
    }

    pub fn set_backup(&mut self, state_bytes: Vec<u8>) {
        let len = state_bytes.len() as u32;

        let memory = self.backup.borrow_mut();

        let mut writer = Writer::new(memory, 0);
        writer.write(&len.to_le_bytes()).unwrap();
        writer.write(&state_bytes).unwrap()
    }

    pub fn write_backup(&mut self, offset: u64, state_bytes: &[u8]) {
        self.backup.write(offset, state_bytes)
    }

    pub fn clear_backup(&mut self) {
        self.set_backup(vec![])
    }

    pub fn read_backup(&self, offset: u64, len: u32) -> Vec<u8> {
        let mut state_bytes = vec![0u8; len as usize];

        self.backup.read(offset, &mut state_bytes);

        state_bytes
    }
}
