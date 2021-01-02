use crate::types::ClassRef;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

type SystemDictionary = Mutex<HashMap<String, ClassRef>>;

static SYS_DIC: Lazy<SystemDictionary> = Lazy::new(|| Mutex::new(HashMap::default()));

pub fn put(key: &[u8], class: ClassRef) {
    debug_assert!(!key.contains(&b'.'));
    let key = Vec::from(key);
    let key = unsafe { String::from_utf8_unchecked(key) };
    let mut dict = SYS_DIC.lock().unwrap();
    dict.insert(key, class);
}

pub fn find(key: &[u8]) -> Option<ClassRef> {
    debug_assert!(!key.contains(&b'.'));
    let key = unsafe {
        std::str::from_utf8_unchecked(key)
    };
    let dict = SYS_DIC.lock().unwrap();
    dict.get(key).cloned()
}
