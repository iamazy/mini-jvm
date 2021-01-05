use classfile::BytesRef;
use std::sync::Arc;
use crate::oops::Oop;

/// A ModuleEntry describes a module that has been defined by a call to JVM_DefineModule.
/// It contains:
///   - Symbol* containing the module's name.
///   - pointer to the java.lang.Module for this module.
///   - pointer to the java.security.ProtectionDomain shared by classes defined to this module.
///   - ClassLoaderData*, class loader of this module.
///   - a growable array containg other module entries that this module can read.
///   - a flag indicating if this module can read all unnamed modules.
///
/// The Mutex Module_lock is shared between ModuleEntry and PackageEntry, to lock either
/// data structure.
#[derive(Default)]
pub struct ModuleEntry {
    // module version number
    pub version: BytesRef,
    // module location
    pub location: BytesRef,
    pub can_read_all_unnamed: bool,
    // JVMTI redefine/retransform support
    pub has_default_read_edges: bool,
    // walk module's reads list at GC safepoints to purge out dead modules
    pub must_walk_read: bool,
    // whether the packages in the module are all unqualifiedly exported
    pub is_open: bool,
    // whether the module is patched via --patch-module
    pub is_patched: bool,
    pub next: Option<Arc<ModuleEntry>>,
}

impl ModuleEntry {
    pub fn name(&self) -> BytesRef {
        unimplemented!()
    }

    pub fn version(&self) -> BytesRef {
        self.version.clone()
    }

    pub fn location(&self) -> BytesRef {
        self.location.clone()
    }

    /// The shared ProtectionDomain reference is set once the VM loads a shared class
    /// originated from the current Module. The referenced ProtectionDomain object is
    /// created by the ClassLoader when loading a class (shared or non-shared) from the
    /// Module for the first time. This ProtectionDomain object is used for all
    /// classes from the Module loaded by the same ClassLoader.
    pub fn shared_protection_domain(&self) -> Oop {
        unimplemented!()
    }

    pub fn can_read(&self) -> bool {
        false
    }

    pub fn has_reads_list(&self) -> bool {
        false
    }

    pub fn add_read(entry: &ModuleEntry) {
        unimplemented!()
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_named(&self) -> bool {
        false
    }

    pub fn can_read_all_unnamed(&self) -> bool {
        assert!(
            self.is_named() || self.can_read_all_unnamed,
            "unnamed modules can always read all unnamed modules"
        );
        self.can_read_all_unnamed
    }

    pub fn has_default_read_edges(&self) -> bool {
        self.has_default_read_edges
    }

    pub fn is_patched(&self) -> bool {
        self.is_patched
    }

    pub fn purge_reads(&self) {
        unimplemented!()
    }

    pub fn delete_reads(&self) {
        unimplemented!()
    }

    pub fn delete_unnamed_module(&self) {
        unimplemented!()
    }

    pub fn next(&self) -> Option<Arc<ModuleEntry>> {
        match &self.next {
            Some(ref module_ptr) => Some((*module_ptr).clone()),
            None => None,
        }
    }
}

pub struct ModuleEntryTable {
    pub entries: Vec<ModuleEntry>,
}

impl ModuleEntryTable {
    pub fn add_entry(&mut self, entry: ModuleEntry) {
        self.entries.push(entry);
    }

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }
}
