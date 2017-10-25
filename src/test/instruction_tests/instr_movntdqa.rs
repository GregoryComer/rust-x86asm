use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 4, 82], OperandSize::Dword)
}

#[test]
fn movntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1729763431, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 52, 221, 103, 24, 26, 103], OperandSize::Qword)
}

