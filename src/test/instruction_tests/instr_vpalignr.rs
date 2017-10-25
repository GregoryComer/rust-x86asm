use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpalignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 15, 231, 74], OperandSize::Dword)
}

#[test]
fn vpalignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1083059379, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 15, 129, 179, 44, 142, 64, 78], OperandSize::Dword)
}

#[test]
fn vpalignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 15, 216, 56], OperandSize::Qword)
}

#[test]
fn vpalignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 72959295, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 15, 172, 75, 63, 69, 89, 4, 62], OperandSize::Qword)
}

#[test]
fn vpalignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 15, 214, 111], OperandSize::Dword)
}

#[test]
fn vpalignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 818706508, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 15, 4, 141, 76, 120, 204, 48, 74], OperandSize::Dword)
}

#[test]
fn vpalignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 15, 217, 38], OperandSize::Qword)
}

#[test]
fn vpalignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1904187915, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 15, 188, 95, 11, 154, 127, 113, 62], OperandSize::Qword)
}

#[test]
fn vpalignr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 101, 143, 15, 234, 34], OperandSize::Dword)
}

#[test]
fn vpalignr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 384854181, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 142, 15, 177, 165, 104, 240, 22, 96], OperandSize::Dword)
}

#[test]
fn vpalignr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM31)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 61, 131, 15, 255, 75], OperandSize::Qword)
}

#[test]
fn vpalignr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RCX, 349071268, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 29, 129, 15, 161, 164, 103, 206, 20, 70], OperandSize::Qword)
}

#[test]
fn vpalignr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 117, 173, 15, 252, 121], OperandSize::Dword)
}

#[test]
fn vpalignr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1792429075, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 77, 170, 15, 52, 85, 19, 76, 214, 106, 81], OperandSize::Dword)
}

#[test]
fn vpalignr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 51, 85, 164, 15, 218, 77], OperandSize::Qword)
}

#[test]
fn vpalignr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 21, 163, 15, 4, 70, 37], OperandSize::Qword)
}

#[test]
fn vpalignr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 206, 15, 238, 16], OperandSize::Dword)
}

#[test]
fn vpalignr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 683228087, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 204, 15, 28, 197, 183, 59, 185, 40, 34], OperandSize::Dword)
}

#[test]
fn vpalignr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM14)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 67, 117, 204, 15, 214, 100], OperandSize::Qword)
}

#[test]
fn vpalignr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 53, 195, 15, 20, 67, 127], OperandSize::Qword)
}

