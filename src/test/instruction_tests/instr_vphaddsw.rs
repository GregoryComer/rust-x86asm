use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 3, 246], OperandSize::Dword)
}

#[test]
fn vphaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 3, 63], OperandSize::Dword)
}

#[test]
fn vphaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 3, 222], OperandSize::Qword)
}

#[test]
fn vphaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 17789595, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 3, 135, 155, 114, 15, 1], OperandSize::Qword)
}

#[test]
fn vphaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 247], OperandSize::Dword)
}

#[test]
fn vphaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 54], OperandSize::Dword)
}

#[test]
fn vphaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 252], OperandSize::Qword)
}

#[test]
fn vphaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 1715480321, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 3, 177, 1, 39, 64, 102], OperandSize::Qword)
}

