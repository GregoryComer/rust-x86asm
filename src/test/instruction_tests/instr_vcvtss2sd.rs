use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 90, 214], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 315816374, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 90, 180, 178, 182, 249, 210, 18], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 90, 235], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 6968297, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 90, 36, 205, 233, 83, 106, 0], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 94, 154, 90, 225], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 297163159, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 86, 142, 90, 163, 151, 89, 182, 17], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 38, 147, 90, 236], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RSI, 595474465, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 102, 132, 90, 190, 33, 56, 126, 35], OperandSize::Qword)
}

