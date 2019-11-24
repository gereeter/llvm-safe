use std::ffi::CStr;
use std::marker::PhantomData;

use libc::c_uint;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
pub use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate};

use inheritance::upcast;
use opaque::Opaque;
use owned::{Owned, DropInPlace};

use llvm::{Context, BasicBlock, Label, Value, Phi, Alloca, Type, FunctionType};

pub struct Builder<'cid: 'context, 'context> {
    _context: PhantomData<&'context Context<'cid>>,
    _opaque: Opaque
}

impl<'cid, 'context> DropInPlace for Builder<'cid, 'context> {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeBuilder(self.as_raw());
    }
}

impl<'cid, 'context> Builder<'cid, 'context> {
    pub fn new(context: &'context Context<'cid>) -> Owned<Builder<'cid, 'context>> {
        unsafe {
            Owned::from_raw(
                LLVMCreateBuilderInContext(context.as_raw()) as *mut Builder
            )
        }
    }

    pub fn position_at_end<'mid: 'block, 'fid: 'block, 'block, 'builder>(&'builder mut self, block: &'block mut BasicBlock<'cid, 'mid, 'fid>) -> &'builder mut PositionedBuilder<'cid, 'context, 'mid, 'fid, 'block> {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
            &mut *(self as *mut Builder as *mut PositionedBuilder)
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const Builder as *mut Builder as LLVMBuilderRef
    }
}

pub struct PositionedBuilder<'cid: 'context, 'context: 'block, 'mid: 'block, 'fid: 'block, 'block> {
    _block: PhantomData<&'block mut BasicBlock<'cid, 'mid, 'fid>>,
    _builder: PhantomData<Builder<'cid, 'context>>,
    _opaque: Opaque
}

macro_rules! binop_impl {
    ( $($(#[$doc:meta])* $rust_name:ident, $c_name:ident)* )  => { $(
        $(#[$doc])*
        pub fn $rust_name(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
            unsafe {
                &*($c_name(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
            }
        }
    )* };
}

macro_rules! cast_impl {
    ( $(#[$doc:meta])* $rust_name:ident, $c_name:ident )  => {
        $(#[$doc])*
        pub fn $rust_name(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
            unsafe {
                &*($c_name(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
            }
        }
    };
}

impl<'cid, 'context, 'mid, 'fid, 'block> PositionedBuilder<'cid, 'context, 'mid, 'fid, 'block> {
    /// Creates an unconditional [`br label <target>`][langref] instruction.
    ///
    /// Corresponds to `CreateBr` ([C++][C++]) and `LLVMBuildBr` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#br-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#aa7cdfc8d05a480d276dede8645bde46d
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga61def0c0fc591008bc9ec04ffc601093
    /// [Rust]: LLVMBuildBr
    pub fn br(&mut self, target: &Label<'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildBr(self.as_raw(), target.as_raw()) as *const Value)
        }
    }

    /// Creates a conditional [`br <cond>, <true_dest>, <false_dest>`][langref] instruction.
    ///
    /// Corresponds to `CreateCondBr` ([C++][C++]) and `LLVMBuildCondBr` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#br-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a3393497feaca1880ab3168ee3db1d7a4
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gaaa5498fef5a2da8016c2cc1278c41c51
    /// [Rust]: LLVMBuildCondBr
    pub fn cond_br(&mut self, cond: &Value<'cid, 'mid, 'fid>, true_dest: &Label<'fid>, false_dest: &Label<'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildCondBr(self.as_raw(), cond.as_raw(), true_dest.as_raw(), false_dest.as_raw()) as *const Value)
        }
    }

binop_impl!{
    /// Creates an [`add`][langref] instruction.
    ///
    /// Corresponds to `CreateAdd` ([C++][C++]) and `LLVMBuildAdd` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#add-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#af6222f26949daf4f0eceaa67d93de274
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga5e20ba4e932d72d97a69e07ff54cfa81
    /// [Rust]: LLVMBuildAdd
    add, LLVMBuildAdd

    /// Creates an [`add nsw`][langref] instruction.
    ///
    /// Corresponds to `CreateAddNSW` ([C++][C++]) and `LLVMBuildNSWAdd` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#add-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a3a4cdfc234f4b873b01f53e0caeaaa11
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga901f44572f6ef9f67e5f7bb496806280
    /// [Rust]: LLVMBuildNSWAdd
    add_nsw, LLVMBuildNSWAdd

    /// Creates an [`add nuw`][langref] instruction.
    ///
    /// Corresponds to `CreateNUWAdd` ([C++][C++]) and `LLVMBuildNUWAdd` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#add-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ae3f594dfe41ae2ed4e54808784159c00
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gac46472d120e1a0d83222509e4e418ac2
    /// [Rust]: LLVMBuildNUWAdd
    add_nuw, LLVMBuildNUWAdd

    /// Creates an [`fadd`][langref] instruction.
    ///
    /// Corresponds to `CreateFAdd` ([C++][C++]) and `LLVMBuildFAdd` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#fadd-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a0878a27006251ab2e48984206df19d84
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga18ffa5e08ab18e49ad9c4f45d69c36e7
    /// [Rust]: LLVMBuildFAdd
    fadd, LLVMBuildFAdd

    /// Creates a [`sub`][langref] instruction.
    ///
    /// Corresponds to `CreateSub` ([C++][C++]) and `LLVMBuildSub` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#sub-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ae72ab55d5f3945b4f302b365e0573b4f
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gac3555db209d44f2750000083c1b64d7d
    /// [Rust]: LLVMBuildSub
    sub, LLVMBuildSub

    /// Creates a [`sub nsw`][langref] instruction.
    ///
    /// Corresponds to `CreateNSWSub` ([C++][C++]) and `LLVMBuildNSWSub` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#sub-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ad5fce5a5e563b12147f6e75f6e560135
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga901f44572f6ef9f67e5f7bb496806280
    /// [Rust]: LLVMBuildNSWSub
    sub_nsw, LLVMBuildNSWSub

    /// Creates a [`sub nuw`][langref] instruction.
    ///
    /// Corresponds to `CreateNUWSub` ([C++][C++]) and `LLVMBuildNUWSub` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#sub-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#af967385862fe4e50eafdf752c37f428b
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gac46472d120e1a0d83222509e4e418ac2
    /// [Rust]: LLVMBuildNUWSub
    sub_nuw, LLVMBuildNUWSub

    /// Creates an [`fsub`][langref] instruction.
    ///
    /// Corresponds to `CreateFSub` ([C++][C++]) and `LLVMBuildFSub` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#fsub-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ac574f9f74f0a8f34461a1d530990d24b
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga6d37daa2fcb7c972cb69c50dce528d36
    /// [Rust]: LLVMBuildFSub
    fsub, LLVMBuildFSub

    /// Creates a [`mul`][langref] instruction.
    ///
    /// Corresponds to `CreateMul` ([C++][C++]) and `LLVMBuildMul` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#mul-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a809645da484eb129d5ffcc78e1468cbc
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga3d3427aa7e6083a809ef7a4dcabfed84
    /// [Rust]: LLVMBuildMul
    mul, LLVMBuildMul

    /// Creates a [`mul nsw`][langref] instruction.
    ///
    /// Corresponds to `CreateNSWMul` ([C++][C++]) and `LLVMBuildNSWMul` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#mul-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#afdc759da98a5392084a258daebf10d32
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga620c57b22a134350df2cce5e676d2b88
    /// [Rust]: LLVMBuildNSWMul
    mul_nsw, LLVMBuildNSWMul

    /// Creates a [`mul nuw`][langref] instruction.
    ///
    /// Corresponds to `CreateNUWMul` ([C++][C++]) and `LLVMBuildNUWMul` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#mul-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a15ab751a46df3451afb0a7a3415cfdf2
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gaf894b21c4ee3c3b9a55626d5038c670d
    /// [Rust]: LLVMBuildNUWMul
    mul_nuw, LLVMBuildNUWMul

    /// Creates an [`fmul`][langref] instruction.
    ///
    /// Corresponds to `CreateFMul` ([C++][C++]) and `LLVMBuildFMul` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#fmul-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a179d92834e61875dbe397728553af68f
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga1a1136d38e745260a9384d6b79ff1149
    /// [Rust]: LLVMBuildFMul
    fmul, LLVMBuildFMul

    /// Creates a [`udiv`][langref] instruction.
    ///
    /// Corresponds to `CreateUDiv` ([C++][C++]) and `LLVMBuildUDiv` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#udiv-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a52bf095aec0b2b57f380e8fb6bff1b24
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga96a9d924ada782f56349cfaefc79344f
    /// [Rust]: LLVMBuildUDiv
    udiv, LLVMBuildUDiv

    /// Creates a [`udiv exact`][langref] instruction.
    ///
    /// Corresponds to `CreateExactUDiv` ([C++][C++]) and `LLVMBuildExactUDiv` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#udiv-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a758eca7c8c704b426769d17353664ecf
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga52e61dbc147dc663d508e25a5c2b9b89
    /// [Rust]: LLVMBuildExactUDiv
    udiv_exact, LLVMBuildExactUDiv

    /// Creates an [`sdiv`][langref] instruction.
    ///
    /// Corresponds to `CreateSDiv` ([C++][C++]) and `LLVMBuildSDiv` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#sdiv-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#aa023b5304a48e37712f2cec064b9ae50
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga99af54457bbe69f1ce275b93881b496d
    /// [Rust]: LLVMBuildSDiv
    sdiv, LLVMBuildSDiv

    /// Creates an [`sdiv exact`][langref] instruction.
    ///
    /// Corresponds to `CreateExactSDiv` ([C++][C++]) and `LLVMBuildExactSDiv` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#sdiv-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a32e509c996d58df8d8f910164437f096
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga4215f84218c9d06e0e8a4d4718d02652
    /// [Rust]: LLVMBuildExactSDiv
    sdiv_exact, LLVMBuildExactSDiv

    /// Creates an [`fdiv`][langref] instruction.
    ///
    /// Corresponds to `CreateFDiv` ([C++][C++]) and `LLVMBuildFDiv` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#fdiv-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#abf156f5610c0dd8fae21230aac333c01
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga110c2b670e7190b0d642c5019049632f
    /// [Rust]: LLVMBuildFDiv
    fdiv, LLVMBuildFDiv


    /// Creates a [`urem`][langref] instruction.
    ///
    /// Corresponds to `CreateURem` ([C++][C++]) and `LLVMBuildURem` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#urem-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a4ef70bab263e38c5e0b8c1bf95a5d814
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga8031af15dd82f8d99029d73a3efb6458
    /// [Rust]: LLVMBuildURem
    urem, LLVMBuildURem

    /// Creates an [`srem`][langref] instruction.
    ///
    /// Corresponds to `CreateSRem` ([C++][C++]) and `LLVMBuildSRem` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#srem-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a83d716924c9612844825c971dfa37677
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga66845502cea8f7e71bba6afda4681461
    /// [Rust]: LLVMBuildSRem
    srem, LLVMBuildSRem

    /// Creates an [`frem`][langref] instruction.
    ///
    /// Corresponds to `CreateFRem` ([C++][C++]) and `LLVMBuildFRem` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#frem-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a22983adbba8de3386ffec124e3d754b5
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga3461da41ca02505baa785beadace31a7
    /// [Rust]: LLVMBuildFRem
    frem, LLVMBuildFRem

    /// Creates a [`shl`][langref] instruction.
    ///
    /// Corresponds to `CreateShl` ([C++][C++]) and `LLVMBuildShl` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#shl-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a009ae136d22248354227df4e67906b46
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga2f643b62b42b1c85959478bc6ccf99d0
    /// [Rust]: LLVMBuildShl
    shl, LLVMBuildShl

    /// Creates a [`lshr`][langref] instruction.
    ///
    /// Corresponds to `CreateLShr` ([C++][C++]) and `LLVMBuildLShr` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#lshr-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a791f659bc62d2c5785b08e84dfe2b29f
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gae24e9eff779495c0d8e28931eeb3fe24
    /// [Rust]: LLVMBuildLShr
    lshr, LLVMBuildLShr

    /// Creates an [`ashr`][langref] instruction.
    ///
    /// Corresponds to `CreateAShr` ([C++][C++]) and `LLVMBuildAShr` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#ashr-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a6d768bd13063a7cf2cf46faa7267e877
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga4e8cc55641452384f508d8caf2a64511
    /// [Rust]: LLVMBuildAShr
    ashr, LLVMBuildAShr

    /// Creates an [`and`][langref] instruction.
    ///
    /// Corresponds to `CreateAnd` ([C++][C++]) and `LLVMBuildAnd` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#and-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a840336af9ba351eb8c51640544519d64
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga0adef253020901c9388bffaaa0836905
    /// [Rust]: LLVMBuildAnd
    and, LLVMBuildAnd

    /// Creates an [`or`][langref] instruction.
    ///
    /// Corresponds to `CreateOr` ([C++][C++]) and `LLVMBuildOr` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#or-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ab1475cfd218c3655256eec53a9d6b1dd
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga849f89e23af190ff50e8b3e680d0e0cd
    /// [Rust]: LLVMBuildOr
    or, LLVMBuildOr

    /// Creates an [`xor`][langref] instruction.
    ///
    /// Corresponds to `CreateXor` ([C++][C++]) and `LLVMBuildXor` ([C][C], [Rust][Rust]).
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#xor-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a56457072cdd2d780e97f55a989d963eb
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gaacb2632bee364c5388e2cb9ab897b988
    /// [Rust]: LLVMBuildXor
    xor, LLVMBuildXor
}

    pub fn neg(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fneg(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn not(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildNot(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn icmp(&mut self, pred: LLVMIntPredicate, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildICmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fcmp(&mut self, pred: LLVMRealPredicate, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFCmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    cast_impl!{ trunc,    LLVMBuildTrunc }
    cast_impl!{ fp_trunc, LLVMBuildFPTrunc }

    cast_impl!{ zext,   LLVMBuildZExt }
    cast_impl!{ sext,   LLVMBuildSExt }
    cast_impl!{ fp_ext, LLVMBuildFPExt }

    cast_impl!{ fp_to_ui,   LLVMBuildFPToUI }
    cast_impl!{ fp_to_si,   LLVMBuildFPToSI }
    cast_impl!{ ui_to_fp,   LLVMBuildUIToFP }
    cast_impl!{ si_to_fp,   LLVMBuildSIToFP }
    cast_impl!{ ptr_to_int, LLVMBuildPtrToInt }
    cast_impl!{ int_to_ptr, LLVMBuildIntToPtr }
    cast_impl!{ bitcast,    LLVMBuildBitCast }

    pub fn get_element_ptr(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildGEP2(self.as_raw(), ty.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn get_element_ptr_in_bounds(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildInBoundsGEP2(self.as_raw(), ty.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn memset(&mut self, ptr: &Value<'cid, 'mid, 'fid>, value: &Value<'cid, 'mid, 'fid>, len: &Value<'cid, 'mid, 'fid>, align: c_uint) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemSet(self.as_raw(), ptr.as_raw(), value.as_raw(), len.as_raw(), align) as *mut Value)
        }
    }

    pub fn memcpy(&mut self, dest: &Value<'cid, 'mid, 'fid>, dest_align: c_uint, src: &Value<'cid, 'mid, 'fid>, src_align: c_uint, size: &Value<'cid, 'mid, 'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemCpy(self.as_raw(), dest.as_raw(), dest_align, src.as_raw(), src_align, size.as_raw()) as *mut Value)
        }
    }

    pub fn memmove(&mut self, dest: &Value<'cid, 'mid, 'fid>, dest_align: c_uint, src: &Value<'cid, 'mid, 'fid>, src_align: c_uint, size: &Value<'cid, 'mid, 'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemMove(self.as_raw(), dest.as_raw(), dest_align, src.as_raw(), src_align, size.as_raw()) as *mut Value)
        }
    }

    pub fn alloca(&mut self, ty: &Type<'cid>, name: &CStr) -> &'block mut Alloca<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildAlloca(self.as_raw(), ty.as_raw(), name.as_ptr()) as *mut Alloca)
        }
    }

    pub fn array_alloca(&mut self, ty: &Type<'cid>, len: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block mut Alloca<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildArrayAlloca(self.as_raw(), ty.as_raw(), len.as_raw(), name.as_ptr()) as *mut Alloca)
        }
    }

    pub fn load(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildLoad2(self.as_raw(), ty.as_raw(), ptr.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn store(&mut self, value: &Value<'cid, 'mid, 'fid>, ptr: &Value<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMBuildStore(self.as_raw(), value.as_raw(), ptr.as_raw());
        }
    }

    pub fn phi(&mut self, ty: &Type<'cid>, name: &CStr) -> &'block mut Phi<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildPhi(self.as_raw(), ty.as_raw(), name.as_ptr()) as *mut Phi)
        }
    }

    /// Creates a [`call`][langref] instruction.
    ///
    /// Corresponds to `CreateCall` ([C++][C++]) and `LLVMBuildCall2` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#call-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#a2f2b90f6238b8dd8ffd39ec6b05f5772
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#ga821864790c90dc5193078c4e17b8cb09
    /// [Rust]: LLVMBuildCall2
    pub fn call(&mut self, ty: &FunctionType<'cid>, func: &Value<'cid, 'mid, 'fid>, args: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildCall2(self.as_raw(), upcast::<_,Type>(ty).as_raw(), func.as_raw(), args.as_ptr() as *const LLVMValueRef as *mut LLVMValueRef, args.len() as u32, name.as_ptr()) as *const Value)
        }
    }

    /// Creates a [`ret <value>`][langref] instruction.
    ///
    /// Corresponds to `CreateRet` ([C++][C++]) and `LLVMBuildRet` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#ret-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ab87f1be0c94c05973a02a06a915e76f7
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gae4c870d69f9787fe98a824a634473155
    /// [Rust]: LLVMBuildRet
    pub fn ret(&mut self, value: &Value<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMBuildRet(self.as_raw(), value.as_raw());
        }
    }

    /// Creates a [`ret void`][langref] instruction.
    ///
    /// Corresponds to `CreateRetVoid` ([C++][C++]) and `LLVMBuildRetVoid` ([C][C], [Rust][Rust])
    ///
    /// [langref]: https://releases.llvm.org/8.0.1/docs/LangRef.html#ret-instruction
    /// [C++]: https://llvm.org/doxygen/classllvm_1_1IRBuilder.html#ae609dbf45ff5cbc2a5b3ddab636bb6f5o
    /// [C]: https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html#gae3b02fabccde1cfd695b48952d9f4229
    /// [Rust]: LLVMBuildRetVoid
    pub fn ret_void(&mut self) {
        unsafe {
            LLVMBuildRetVoid(self.as_raw());
        }
    }


    pub fn unreachable(&mut self) {
        unsafe {
            LLVMBuildUnreachable(self.as_raw());
        }
    }


    pub fn get_position(&self) -> &'block Label<'fid> {
        unsafe {
            &*(LLVMGetInsertBlock(self.as_raw()) as *mut Label as *const Label)
        }
    }

    pub fn position_at_end(&mut self, block: &'block mut BasicBlock<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const PositionedBuilder as *mut PositionedBuilder as LLVMBuilderRef
    }
}
