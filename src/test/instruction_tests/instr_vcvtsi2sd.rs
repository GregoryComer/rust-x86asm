use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 42, 254], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1681930907, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 42, 182, 155, 58, 64, 100], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 42, 226], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 42, 28, 113], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 243, 42, 217], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1960113054, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 219, 42, 180, 83, 158, 243, 212, 116], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 42, 197], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1150078013, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 172, 247, 61, 204, 140, 68], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 19, 42, 219], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 87, 8, 42, 48], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 223, 48, 42, 253], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 143, 8, 42, 48], OperandSize::Qword)
}

