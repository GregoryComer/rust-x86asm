use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtph2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 246], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 20, 130], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 228], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 515665005, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 161, 109, 108, 188, 30], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 245], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1027929664, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 148, 150, 64, 246, 68, 61], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 215], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 51], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 19, 206], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 19, 12, 72], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 140, 19, 250], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 19, 36, 65], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 19, 243], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 19, 12, 114], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 172, 19, 222], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1032829517, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 169, 19, 148, 88, 77, 186, 143, 61], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 19, 221], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 19, 58], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 125, 159, 19, 223], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 19, 62], OperandSize::Qword)
}

