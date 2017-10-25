use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 238], OperandSize::Dword)
}

#[test]
fn sqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 752036482, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 52, 181, 130, 42, 211, 44], OperandSize::Dword)
}

#[test]
fn sqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 253], OperandSize::Qword)
}

#[test]
fn sqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 81, 43], OperandSize::Qword)
}

