use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 2, 233], OperandSize::Dword)
}

#[test]
fn vphaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1384183446, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 2, 156, 134, 150, 246, 128, 82], OperandSize::Dword)
}

#[test]
fn vphaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 2, 248], OperandSize::Qword)
}

#[test]
fn vphaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 349378036, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 2, 132, 251, 244, 21, 211, 20], OperandSize::Qword)
}

#[test]
fn vphaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 2, 250], OperandSize::Dword)
}

#[test]
fn vphaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 2, 7], OperandSize::Dword)
}

#[test]
fn vphaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 2, 218], OperandSize::Qword)
}

#[test]
fn vphaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 2105564607, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 2, 172, 89, 191, 93, 128, 125], OperandSize::Qword)
}

