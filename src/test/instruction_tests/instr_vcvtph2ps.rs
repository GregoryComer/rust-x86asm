use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtph2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 224], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 525467871, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 164, 255, 223, 0, 82, 31], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 220], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 1410734342, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 128, 6, 25, 22, 84], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 209], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1283727202, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 60, 189, 98, 31, 132, 76], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 249], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 55771028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 172, 207, 148, 255, 82, 3], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 19, 250], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 943682303, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 19, 60, 213, 255, 114, 63, 56], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 141, 19, 200], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectDisplaced(RAX, 1221966051, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 141, 19, 128, 227, 184, 213, 72], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 19, 249], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 621617265, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 19, 153, 113, 32, 13, 37], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 170, 19, 250], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 19, 36, 142], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 19, 213], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDX, 43929770, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 19, 170, 170, 80, 158, 2], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 155, 19, 252], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 147797563, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 19, 36, 125, 59, 54, 207, 8], OperandSize::Qword)
}

