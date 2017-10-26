use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 140, 122, 211], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1797704361, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 122, 12, 213, 169, 202, 38, 107], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 255, 142, 122, 247], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 2059637312, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 255, 140, 122, 172, 206, 64, 146, 195, 122], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 122, 196], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 756891667, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 122, 4, 197, 19, 64, 29, 45], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 255, 170, 122, 214], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RCX, 1012126027, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 255, 175, 122, 137, 75, 209, 83, 60], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 251, 122, 203], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 630847802, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 203, 122, 28, 149, 58, 249, 153, 37], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 255, 250, 122, 213], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RSI, 1698279329, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 122, 134, 161, 175, 57, 101], OperandSize::Qword)
}

