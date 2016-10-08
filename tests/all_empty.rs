extern crate llvm_safe;

use std::mem;

use llvm_safe::llvm::{BasicBlock, Label, Builder, PositionedBuilder, Constant, Context, Function, FunctionLabel, Module, Phi, Target, TargetMachine, Type, Value};

#[test]
fn test_all_empty() {
    assert_eq!(mem::size_of::<BasicBlock<'static, 'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<Label<'static>>(), 0);
    assert_eq!(mem::size_of::<Builder<'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<PositionedBuilder<'static, 'static, 'static, 'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<Constant<'static>>(), 0);
    assert_eq!(mem::size_of::<Context<'static>>(), 0);
    assert_eq!(mem::size_of::<Function<'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<FunctionLabel<'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<Module<'static, 'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<Phi<'static, 'static, 'static>>(), 0);
    assert_eq!(mem::size_of::<Target>(), 0);
    assert_eq!(mem::size_of::<TargetMachine>(), 0);
    assert_eq!(mem::size_of::<Type<'static>>(), 0);
    assert_eq!(mem::size_of::<Value<'static, 'static, 'static>>(), 0);

    assert_eq!(mem::align_of::<BasicBlock<'static, 'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<Label<'static>>(), 1);
    assert_eq!(mem::align_of::<Builder<'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<PositionedBuilder<'static, 'static, 'static, 'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<Constant<'static>>(), 1);
    assert_eq!(mem::align_of::<Context<'static>>(), 1);
    assert_eq!(mem::align_of::<Function<'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<FunctionLabel<'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<Module<'static, 'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<Phi<'static, 'static, 'static>>(), 1);
    assert_eq!(mem::align_of::<Target>(), 1);
    assert_eq!(mem::align_of::<TargetMachine>(), 1);
    assert_eq!(mem::align_of::<Type<'static>>(), 1);
    assert_eq!(mem::align_of::<Value<'static, 'static, 'static>>(), 1);
}
