use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 90, 192], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 90, 12, 158], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 90, 246], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 90, 12, 139], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 118, 159, 90, 207], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 1061888318, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 70, 138, 90, 168, 62, 33, 75, 63], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 14, 159, 90, 234], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1577366397, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 14, 135, 90, 12, 149, 125, 179, 4, 94], OperandSize::Qword)
}

