use alloc::{collections::BTreeMap, string::String, sync::Arc};
use spin::RwLock;
use x86_64::{
    structures::paging::{page::PageRange, Page},
    VirtAddr,
};

use crate::resource::*;

use super::*;
use sync::SemaphoreSet;

#[derive(Debug, Clone)]
pub struct ProcessData {
    // shared data
    pub(super) env: Arc<RwLock<BTreeMap<String, String>>>,

    // process specific data
    pub(super) stack_segment: Option<PageRange>,

    // file descriptors table
    pub(super) resources: Arc<RwLock<ResourceSet>>,

    // the number of page that code segment is mapped
    pub(super) code_segment_pages: u64,

    // semaphores
    pub(super) semaphores: Arc<RwLock<SemaphoreSet>>,
}

impl Default for ProcessData {
    fn default() -> Self {
        Self {
            env: Arc::new(RwLock::new(BTreeMap::new())),
            stack_segment: None,
            resources: Arc::new(RwLock::new(ResourceSet::default())),
            code_segment_pages: 0,
            semaphores: Arc::new(RwLock::new(SemaphoreSet::new())),
        }
    }
}

impl ProcessData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn env(&self, key: &str) -> Option<String> {
        self.env.read().get(key).cloned()
    }

    pub fn set_env(&mut self, key: &str, val: &str) {
        self.env.write().insert(key.into(), val.into());
    }

    pub fn set_stack(&mut self, start: VirtAddr, size: u64) {
        let start = Page::containing_address(start);
        self.stack_segment = Some(Page::range(start, start + size));
    }

    pub fn is_on_stack(&self, addr: VirtAddr) -> bool {
        VirtAddr::new(addr.as_u64() & STACK_START_MASK)
            == self.stack_segment.unwrap().end.start_address() - STACK_MAX_SIZE
    }

    pub fn read(&self, fd: u8, buf: &mut [u8]) -> isize {
        self.resources.read().read(fd, buf)
    }

    pub fn write(&self, fd: u8, buf: &[u8]) -> isize {
        self.resources.read().write(fd, buf)
    }

    pub fn sem_wait(&self, key: u32, pid: ProcessId) -> SemaphoreResult {
        self.semaphores.write().wait(key, pid)
    }

    pub fn sem_signal(&self, key: u32) -> SemaphoreResult {
        self.semaphores.write().signal(key)
    }

    pub fn new_sem(&self, key: u32, value: usize) -> bool {
        self.semaphores.write().insert(key, value)
    }

    pub fn remove_sem(&self, key: u32) -> bool {
        self.semaphores.write().remove(key)
    }
}
