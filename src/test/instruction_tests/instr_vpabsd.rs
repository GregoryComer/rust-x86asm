use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 255], OperandSize::Dword)
}

#[test]
fn vpabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 25], OperandSize::Dword)
}

#[test]
fn vpabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 255], OperandSize::Qword)
}

#[test]
fn vpabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RAX, 935912060, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 136, 124, 226, 200, 55], OperandSize::Qword)
}

#[test]
fn vpabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 246], OperandSize::Dword)
}

#[test]
fn vpabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDX, 77797764, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 178, 132, 25, 163, 4], OperandSize::Dword)
}

#[test]
fn vpabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 200], OperandSize::Qword)
}

#[test]
fn vpabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 44, 89], OperandSize::Qword)
}

