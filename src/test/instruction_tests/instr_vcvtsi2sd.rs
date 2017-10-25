use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 42, 222], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 915104849, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 42, 177, 81, 100, 139, 54], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 42, 251], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 925019906, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 42, 160, 2, 175, 34, 55], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 243, 42, 228], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 243, 42, 52, 202], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 42, 219], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 143214393, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 42, 180, 121, 57, 71, 137, 8], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 210], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 0, 42, 9], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 191, 24, 42, 213], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 179, 42, 4, 127], OperandSize::Qword)
}

