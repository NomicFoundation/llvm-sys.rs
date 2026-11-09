//! The module/file/archive linker

use super::prelude::*;


#[repr(C)]
#[derive(Debug)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    #[deprecated(since = "37.0.0", note = "LLVMLinkerPreserveSource has no effect")]
    LLVMLinkerPreserveSource_Removed = 1,
}

/// A library whose 20-byte address is known.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMEVMLibrarySymbol {
    pub name: *const ::libc::c_char,
    pub value: [::libc::c_char; 20],
}

/// Everything `LLVMAssembleEVM` produces.
#[repr(C)]
#[derive(Debug)]
pub struct LLVMEVMAssembleResult {
    pub bytecode: LLVMMemoryBufferRef,
    pub elf_object: LLVMMemoryBufferRef,
    pub unresolved_names: *mut *mut ::libc::c_char,
    pub unresolved_offsets: *mut u64,
    pub num_unresolved: u64,
}

extern "C" {
    /// Link the source module into the destination module.
    ///
    /// Destroys the source module, returns true on error. Use the diagnostic
    /// handler to get any diagnostic message.
    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;

    /// Returns EVM immutables and their offsets of the ELF object file passed in `InMemBuf`.
    pub fn LLVMGetImmutablesEVM(
        InMemBuf: LLVMMemoryBufferRef,
        ImmutableIDs: *mut *mut *mut ::libc::c_char,
        ImmutableOffsets: *mut *mut u64,
    ) -> u64;

    /// Assembles the EVM dependencies with the main module, appending
    /// `Metadata` to the code and resolving the library addresses given in
    /// `LibrarySymbols`. Fills `Result` with the bytecode, the relocatable
    /// ELF object when `EmitElfObject` is set, and one (name, offset) pair per
    /// library reference left unresolved and zero-filled. Returns the result
    /// by value.
    pub fn LLVMAssembleEVM(
        InMemBufs: *const LLVMMemoryBufferRef,
        InMemBufIDs: *const *const ::libc::c_char,
        NumInBufs: u64,
        Metadata: *const ::libc::c_char,
        MetadataSize: u64,
        LibrarySymbols: *const LLVMEVMLibrarySymbol,
        NumLibrarySymbols: u64,
        EmitElfObject: LLVMBool,
    ) -> LLVMEVMAssembleResult;

    /// Releases every field of `Result` that is not null.
    pub fn LLVMDisposeAssembleResultEVM(Result: *mut LLVMEVMAssembleResult);

    /// Dispose immutable names and their offsets returned by LLVMGetImmutablesEVM.
    pub fn LLVMDisposeImmutablesEVM(
        ImmutableIDs: *const *const ::libc::c_char,
        ImmutableOffsets: *const u64,
        NumOfImmutables: u64,
    );

}
