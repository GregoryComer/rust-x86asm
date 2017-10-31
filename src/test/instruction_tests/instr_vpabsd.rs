use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 224], OperandSize::Dword)
}

#[test]
fn vpabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 49], OperandSize::Dword)
}

#[test]
fn vpabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 234], OperandSize::Qword)
}

#[test]
fn vpabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 52, 88], OperandSize::Qword)
}

#[test]
fn vpabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 193], OperandSize::Dword)
}

#[test]
fn vpabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(ECX, 462925463, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 161, 151, 174, 151, 27], OperandSize::Dword)
}

#[test]
fn vpabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 194], OperandSize::Qword)
}

#[test]
fn vpabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RCX, 942843888, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 177, 240, 167, 50, 56], OperandSize::Qword)
}

