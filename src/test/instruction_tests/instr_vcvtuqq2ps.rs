use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 122, 217], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 122, 60, 255], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 255, 141, 122, 220], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1481708568, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 255, 140, 122, 148, 154, 24, 20, 81, 88], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 122, 222], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 767408389, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 122, 174, 5, 185, 189, 45], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 255, 171, 122, 245], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1020598732, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 122, 148, 118, 204, 25, 213, 60], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 189, 122, 220], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 122, 44, 179], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 220, 122, 250], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 255, 202, 122, 44, 118], OperandSize::Qword)
}

