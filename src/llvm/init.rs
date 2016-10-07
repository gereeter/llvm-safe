use llvm_sys::target::{LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargets, LLVM_InitializeAllTargetMCs, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllAsmParsers};

pub unsafe fn init_target_infos() {
    LLVM_InitializeAllTargetInfos()
}

pub unsafe fn init_targets() {
    LLVM_InitializeAllTargets()
}

pub unsafe fn init_target_mcs() {
    LLVM_InitializeAllTargetMCs()
}

pub unsafe fn init_asm_printers() {
    LLVM_InitializeAllAsmPrinters()
}

pub unsafe fn init_asm_parsers() {
    LLVM_InitializeAllAsmParsers()
}
