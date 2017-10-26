use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 221], OperandSize::Dword)
}

#[test]
fn mulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 11], OperandSize::Dword)
}

#[test]
fn mulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 255], OperandSize::Qword)
}

#[test]
fn mulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 405797413, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 188, 158, 37, 250, 47, 24], OperandSize::Qword)
}

