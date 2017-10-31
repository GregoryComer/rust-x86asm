use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 90, 254], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 754689056, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 90, 137, 32, 164, 251, 44], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 90, 254], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1325981068, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 90, 44, 141, 140, 221, 8, 79], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 94, 154, 90, 249], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1253763083, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 78, 137, 90, 148, 241, 11, 232, 186, 74], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 6, 150, 90, 204], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 133, 90, 60, 82], OperandSize::Qword)
}

