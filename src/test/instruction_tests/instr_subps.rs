use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 210], OperandSize::Dword)
}

#[test]
fn subps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 35196781, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 4, 181, 109, 15, 25, 2], OperandSize::Dword)
}

#[test]
fn subps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 248], OperandSize::Qword)
}

#[test]
fn subps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 44, 216], OperandSize::Qword)
}

