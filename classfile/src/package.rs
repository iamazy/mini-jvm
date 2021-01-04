use crate::module::{ModuleEntryTable, ModuleEntry};
use std::sync::Arc;
use crate::BytesRef;

/// A PackageEntry basically represents a Java package.  It contains:
///   - Symbol* containing the package's name.
///   - ModuleEntry* for this package's containing module.
///   - a field indicating if the package is exported unqualifiedly or to all
///     unnamed modules.
///   - a growable array containing other module entries that this
///     package is exported to.
///
/// Packages can be exported in the following 3 ways:
///   - not exported:        the package does not have qualified or unqualified exports.
///   - qualified exports:   the package has been explicitly qualified to at least
///                            one particular module or has been qualifiedly exported
///                            to all unnamed modules.
///                            Note: being exported to all unnamed is a form of a qualified
///                            export. It is equivalent to the package being explicitly
///                            exported to all current and future unnamed modules.
///   - unqualified exports: the package is exported to all modules.
///
/// A package can transition from:
///   - being not exported, to being exported either in a qualified or unqualified manner
///   - being qualifiedly exported, to unqualifiedly exported. Its exported scope is widened.
///
/// A package cannot transition from:
///   - being unqualifiedly exported, to exported qualifiedly to a specific module.
///       This transition attempt is silently ignored in set_exported.
///   - being qualifiedly exported to not exported.
///       Because transitions are only allowed from less exposure to greater exposure,
///       the transition from qualifiedly exported to not exported would be considered
///       a backward direction.  Therefore the implementation considers a package as
///       qualifiedly exported even if its export-list exists but is empty.
///
/// The Mutex Module_lock is shared between ModuleEntry and PackageEntry, to lock either
/// data structure.
/// PKG_EXP_UNQUALIFIED and PKG_EXP_ALLUNNAMED indicate whether the package is
/// exported unqualifiedly or exported to all unnamed modules.  They are used to
/// set the value of _export_flags.  Field _export_flags and the _qualified_exports
/// list are used to determine a package's export state.
/// Valid states are:
///
///   1. Package is not exported
///      _export_flags is zero and _qualified_exports is null
///   2. Package is unqualifiedly exported
///      _export_flags is set to PKG_EXP_UNQUALIFIED
///      _qualified_exports may or may not be null depending on whether the package
///        transitioned from qualifiedly exported to unqualifiedly exported.
///   3. Package is qualifiedly exported
///      _export_flags may be set to PKG_EXP_ALLUNNAMED if the package is also
///        exported to all unnamed modules
///      _qualified_exports will be non-null
///   4. Package is exported to all unnamed modules
///      _export_flags is set to PKG_EXP_ALLUNNAMED
///      _qualified_exports may or may not be null depending on whether the package
///        is also qualifiedly exported to one or more named modules.
pub struct PackageEntry {
    pub module: Arc<ModuleEntry>,
    // Indicates if package is exported unqualifiedly or to all unnamed. Access to
    // this field is protected by the Module_lock.
    pub export_flags: u16,
    // Used to indicate for packages with classes loaded by the boot loader that
    // a class in that package has been loaded.  And, for packages with classes
    // loaded by the boot loader from -Xbootclasspath/a in an unnamed module, it
    // indicates from which class path entry.
    pub classpath_index: u16,
    pub must_walk_exports: bool
}

impl PackageEntry {

    pub fn name(&self) -> BytesRef {
        unimplemented!()
    }

    pub fn classpath_index(&self) -> u16 {
        self.classpath_index
    }

    pub fn has_loaded_class(&self) -> bool {
        self.classpath_index != -1
    }

    pub fn in_unnamed_module(&self) -> bool {
        !self.module.is_named()
    }
}


pub struct PackageEntryTable {
    pub entries: Vec<PackageEntry>,
}

impl PackageEntryTable {

    pub fn add_entry(&mut self, entry: PackageEntry) {
        self.entries.push(entry);
    }

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }
}