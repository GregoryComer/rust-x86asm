use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(IndirectDisplaced(ESI, 1223747755, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 190, 171, 232, 240, 72], OperandSize::Dword)
}

#[test]
fn movntdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQ, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 369938084, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 231, 180, 206, 164, 206, 12, 22], OperandSize::Qword)
}

