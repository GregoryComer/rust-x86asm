use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 216], OperandSize::Dword)
}

#[test]
fn vpabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 20, 127], OperandSize::Dword)
}

#[test]
fn vpabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 230], OperandSize::Qword)
}

#[test]
fn vpabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDX, 852059783, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 170, 135, 102, 201, 50], OperandSize::Qword)
}

#[test]
fn vpabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 197], OperandSize::Dword)
}

#[test]
fn vpabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1844983925, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 12, 245, 117, 56, 248, 109], OperandSize::Dword)
}

#[test]
fn vpabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 220], OperandSize::Qword)
}

#[test]
fn vpabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1719608523, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 172, 200, 203, 36, 127, 102], OperandSize::Qword)
}

