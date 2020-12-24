/// A Symbol is a canonicalized string.
/// All Symbols reside in global SymbolTable and are reference counted.
///
/// Reference counting
///
/// All Symbols are allocated and added to the SymbolTable.
/// When a class is unloaded, the reference counts of the Symbol pointers in
/// the ConstantPool and in InstanceKlass (see release_C_heap_structures) are
/// decremented.  When the reference count for a Symbol goes to 0, the garbage
/// collector can free the Symbol and remove it from the SymbolTable.
///
/// 0) Symbols need to be reference counted when a pointer to the Symbol is
/// saved in persistent storage.  This does not include the pointer
/// in the SymbolTable bucket (the _literal field in HashtableEntry)
/// that points to the Symbol.  All other stores of a Symbol*
/// to a field of a persistent variable (e.g., the _name filed in
/// fieldDescriptor or _ptr in a CPSlot) is reference counted.
///
/// 1) The lookup of a "name" in the SymbolTable either creates a Symbol F for
/// "name" and returns a pointer to F or finds a pre-existing Symbol F for
/// "name" and returns a pointer to it. In both cases the reference count for F
/// is incremented under the assumption that a pointer to F will be created from
/// the return value. Thus the increment of the reference count is on the lookup
/// and not on the assignment to the new Symbol*.  That is
///    Symbol* G = lookup()
///                ^ increment on lookup()
/// and not
///    Symbol* G = lookup()
///              ^ increment on assignment
/// The reference count must be decremented manually when the copy of the
/// pointer G is destroyed.
///
/// 2) For a local Symbol* A that is a copy of an existing Symbol* B, the
/// reference counting is elided when the scope of B is greater than the scope
/// of A.  For example, in the code fragment
/// below "klass" is passed as a parameter to the method.  Symbol* "kn"
/// is a copy of the name in "klass".
///
///   Symbol*  kn = klass->name();
///   unsigned int d_hash = dictionary()->compute_hash(kn, class_loader);
///
/// The scope of "klass" is greater than the scope of "kn" so the reference
/// counting for "kn" is elided.
///
/// Symbol* copied from ConstantPool entries are good candidates for reference
/// counting elision.  The ConstantPool entries for a class C exist until C is
/// unloaded.  If a Symbol* is copied out of the ConstantPool into Symbol* X,
/// the Symbol* in the ConstantPool will in general out live X so the reference
/// counting on X can be elided.
///
/// For cases where the scope of A is not greater than the scope of B,
/// the reference counting is explicitly done.  See ciSymbol,
/// ResolutionErrorEntry and ClassVerifier for examples.
///
/// 3) When a Symbol K is created for temporary use, generally for substrings of
/// an existing symbol or to create a new symbol, assign it to a
/// TempNewSymbol. The SymbolTable methods new_symbol(), lookup()
/// and probe() all potentially return a pointer to a new Symbol.
/// The allocation (or lookup) of K increments the reference count for K
/// and the destructor decrements the reference count.
///
/// This cannot be inherited from ResourceObj because it cannot have a vtable.
/// Since sometimes this is allocated from Metadata, pick a base allocation
/// type without virtual functions.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub length: u16,
    pub body: [u8; 2]
}