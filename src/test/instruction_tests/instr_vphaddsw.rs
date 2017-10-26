use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 3, 248], OperandSize::Dword)
}

#[test]
fn vphaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1824509441, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 3, 182, 1, 206, 191, 108], OperandSize::Dword)
}

#[test]
fn vphaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 3, 199], OperandSize::Qword)
}

#[test]
fn vphaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 3, 19], OperandSize::Qword)
}

#[test]
fn vphaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 216], OperandSize::Dword)
}

#[test]
fn vphaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 280589619, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 3, 12, 93, 51, 117, 185, 16], OperandSize::Dword)
}

#[test]
fn vphaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 3, 217], OperandSize::Qword)
}

#[test]
fn vphaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 3, 24], OperandSize::Qword)
}

