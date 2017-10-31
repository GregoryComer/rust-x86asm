use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 42, 193], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 42, 12, 151], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 213], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1317374078, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 20, 197, 126, 136, 133, 78], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 195, 42, 195], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 473458366, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 219, 42, 52, 221, 190, 102, 56, 28], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 42, 207], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1731113593, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 42, 28, 189, 121, 178, 46, 103], OperandSize::Dword)
}

#[test]
fn vcvtsi2sd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 51, 42, 217], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1380715588, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 43, 42, 172, 94, 68, 12, 76, 82], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 231, 48, 42, 226], OperandSize::Qword)
}

#[test]
fn vcvtsi2sd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 876952989, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 167, 8, 42, 52, 141, 157, 61, 69, 52], OperandSize::Qword)
}

