use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1539745397, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 44, 133, 117, 166, 198, 91], OperandSize::Dword)
}

#[test]
fn movntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 33], OperandSize::Qword)
}

