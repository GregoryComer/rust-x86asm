use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 121, 247], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 21255234, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 121, 164, 159, 66, 84, 68, 1], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 125, 143, 121, 244], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1190855034, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 140, 121, 164, 82, 122, 1, 251, 70], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 121, 240], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1140223862, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 121, 156, 139, 118, 111, 246, 67], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 125, 172, 121, 247], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 174, 121, 12, 130], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 158, 121, 223], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1886994339, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 121, 44, 69, 163, 63, 121, 112], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 125, 154, 121, 210], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 202, 121, 12, 143], OperandSize::Qword)
}

