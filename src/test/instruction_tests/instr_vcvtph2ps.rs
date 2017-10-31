use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtph2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 225], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 504223962, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 156, 137, 218, 216, 13, 30], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 249], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 25], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 214], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 60, 209], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 241], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 933665051, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 132, 142, 27, 153, 166, 55], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 19, 209], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 488834075, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 19, 145, 27, 4, 35, 29], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 125, 139, 19, 236], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 122000218, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 141, 19, 28, 157, 90, 147, 69, 7], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 19, 234], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 144942782, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 19, 44, 189, 190, 166, 163, 8], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 170, 19, 237], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1377429084, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 19, 188, 183, 92, 230, 25, 82], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 19, 213], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDX, 1512101724, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 19, 146, 92, 215, 32, 90], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 125, 159, 19, 255], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 273373493, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 19, 44, 205, 53, 89, 75, 16], OperandSize::Qword)
}

