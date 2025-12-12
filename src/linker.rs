//! The module/file/archive linker

use super::prelude::*;

use crate::target_machine::LLVMTargetMachineRef;

#[repr(C)]
#[derive(Debug)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    #[deprecated(since = "37.0.0", note = "LLVMLinkerPreserveSource has no effect")]
    LLVMLinkerPreserveSource_Removed = 1,
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

    /// Links all EVM dependencies with the main module.
    /// All input buffers must be valid ELF object files.
    pub fn LLVMAssembleEVM(
        CodeSegment: u64,
        InMemBufs: *const LLVMMemoryBufferRef,
        InMemBufIDs: *const *const ::libc::c_char,
        NumInBufs: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Resolves undefined linker symbols in the ELF object file `InMemBuf`.
    /// Returns ELF object file if there remain unresolved linker symbols. Otherwise returns the bytecode.
    pub fn LLVMLinkEVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LinkerSymbolKeys: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: u64,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check whether the EVM memory buffer is a valid ELF binary.
    pub fn LLVMIsELFEVM(InMemBuf: LLVMMemoryBufferRef) -> LLVMBool;

    /// Dispose immutable names and their offsets returned by LLVMGetImmutablesEVM.
    pub fn LLVMDisposeImmutablesEVM(
        ImmutableIDs: *const *const ::libc::c_char,
        ImmutableOffsets: *const u64,
        NumOfImmutables: u64,
    );

    /// Return undefined references from an EVM ELF object.
    pub fn LLVMGetUndefinedReferencesEVM(
        InMemBuf: LLVMMemoryBufferRef,
        LinkerSymbols: *mut *mut *mut ::libc::c_char,
        LinkerSymbolsSize: *mut u64,
    );

    /// Returns an array of offsets for the linker symbol relocations of the ELF object file.
    pub fn LLVMGetSymbolOffsetsEVM(
        InMemBuf: LLVMMemoryBufferRef,
        SymbolName: *const ::libc::c_char,
        SymbolOffsets: *mut *mut u64,
    ) -> u64;

    /// Releases the array of linker symbol offsets.
    pub fn LLVMDisposeSymbolOffsetsEVM(SymbolOffsets: *const u64);

    /// Add metadata to the ELF-wrapped module.
    pub fn LLVMAddMetadata(
        InMemBuf: LLVMMemoryBufferRef,
        MetadataPtr: *const ::libc::c_char,
        MetadataSize: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Dispose the undefined references.
    pub fn LLVMDisposeUndefinedReferences(
        References: *const *const ::libc::c_char,
        ReferencesSize: u64,
    );
}
