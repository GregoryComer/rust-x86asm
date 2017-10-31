use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 122, 243], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 122, 28, 177], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 255, 140, 122, 197], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 141, 122, 52, 72], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 122, 236], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 177168321, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 122, 188, 136, 193, 95, 143, 10], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 255, 175, 122, 223], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 255, 175, 122, 36, 202], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 222, 122, 219], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 674533951, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 122, 188, 199, 63, 146, 52, 40], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 255, 191, 122, 243], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 206, 122, 12, 90], OperandSize::Qword)
}

