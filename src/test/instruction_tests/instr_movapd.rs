use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 220], OperandSize::Dword)
}

#[test]
fn movapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 448262569, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 52, 125, 169, 241, 183, 26], OperandSize::Dword)
}

#[test]
fn movapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 215], OperandSize::Qword)
}

#[test]
fn movapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1823071980, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 139, 236, 222, 169, 108], OperandSize::Qword)
}

#[test]
fn movapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 248], OperandSize::Dword)
}

#[test]
fn movapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 52, 182], OperandSize::Dword)
}

#[test]
fn movapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 230], OperandSize::Qword)
}

#[test]
fn movapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 49], OperandSize::Qword)
}

