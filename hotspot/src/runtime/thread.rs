use crate::oops::Oop;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct JavaThread {
    pub thread_object: Option<Oop>,
    pub exception: Option<Oop>,
    pub is_active: bool,
    pub thread_state: JavaThreadState,
    pub safe_point_state: Option<ThreadSafePointState>
}

#[derive(Debug, Clone)]
pub enum JavaThreadState {
    Uninitialized = 0,
    New = 2,
    NewTrans = 3,
    InNative = 4,
    InNativeTrans = 5,
    InVm = 6,
    InVmTrans = 7,
    InJava = 8,
    InJavaTrans = 9,
    Blocked = 10,
    BlockedTrans = 11,
    MaxState = 12
}

#[derive(Debug, Clone)]
pub struct ThreadSafePointState {
    pub thread: Box<JavaThread>,
    pub safe_point_safe: bool,
    pub safe_point_id: u64,
    pub next: Arc<Box<ThreadSafePointState>>
}