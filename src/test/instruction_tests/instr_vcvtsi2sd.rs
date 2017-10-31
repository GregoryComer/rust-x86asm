use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 42, 194], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1235540274, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 42, 182, 50, 217, 164, 73], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 42, 193], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 42, 27], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 219, 42, 195], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 203, 42, 23], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 42, 202], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1234417047, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 42, 52, 221, 151, 181, 147, 73], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 59, 42, 244], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 44, 121], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 167, 24, 42, 243], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1359309240, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 97, 203, 42, 180, 139, 184, 105, 5, 81], OperandSize::Qword)
}

