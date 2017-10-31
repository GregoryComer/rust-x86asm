use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 208], OperandSize::Dword)
}

#[test]
fn paddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EAX, 849285518, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 144, 142, 17, 159, 50], OperandSize::Dword)
}

#[test]
fn paddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 214], OperandSize::Qword)
}

#[test]
fn paddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1519411931, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 12, 157, 219, 98, 144, 90], OperandSize::Qword)
}

#[test]
fn paddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 198], OperandSize::Dword)
}

#[test]
fn paddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 538915384, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 136, 56, 50, 31, 32], OperandSize::Dword)
}

#[test]
fn paddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 211], OperandSize::Qword)
}

#[test]
fn paddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1086448704, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 132, 209, 64, 228, 193, 64], OperandSize::Qword)
}

