use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 69, 230], OperandSize::Dword)
}

#[test]
fn vpsrlvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 217977450, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 69, 151, 106, 18, 254, 12], OperandSize::Dword)
}

#[test]
fn vpsrlvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 69, 203], OperandSize::Qword)
}

#[test]
fn vpsrlvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1381399647, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 69, 28, 181, 95, 124, 86, 82], OperandSize::Qword)
}

#[test]
fn vpsrlvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 69, 212], OperandSize::Dword)
}

#[test]
fn vpsrlvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 69, 60, 74], OperandSize::Dword)
}

#[test]
fn vpsrlvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 69, 196], OperandSize::Qword)
}

#[test]
fn vpsrlvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 1158518635, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 69, 154, 107, 151, 13, 69], OperandSize::Qword)
}

#[test]
fn vpsrlvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 69, 197], OperandSize::Dword)
}

#[test]
fn vpsrlvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 94761400, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 69, 36, 253, 184, 241, 165, 5], OperandSize::Dword)
}

#[test]
fn vpsrlvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1548222738, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 155, 69, 4, 141, 18, 1, 72, 92], OperandSize::Dword)
}

#[test]
fn vpsrlvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 29, 134, 69, 239], OperandSize::Qword)
}

#[test]
fn vpsrlvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 109, 130, 69, 3], OperandSize::Qword)
}

#[test]
fn vpsrlvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RSI, 1314894936, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 109, 147, 69, 150, 88, 180, 95, 78], OperandSize::Qword)
}

#[test]
fn vpsrlvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 69, 235], OperandSize::Dword)
}

#[test]
fn vpsrlvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 69, 4, 241], OperandSize::Dword)
}

#[test]
fn vpsrlvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 191, 69, 19], OperandSize::Dword)
}

#[test]
fn vpsrlvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 37, 175, 69, 233], OperandSize::Qword)
}

#[test]
fn vpsrlvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 879065092, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 29, 167, 69, 172, 127, 4, 120, 101, 52], OperandSize::Qword)
}

#[test]
fn vpsrlvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 111106832, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 181, 69, 140, 207, 16, 91, 159, 6], OperandSize::Qword)
}

#[test]
fn vpsrlvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 69, 199], OperandSize::Dword)
}

#[test]
fn vpsrlvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1989630858, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 69, 140, 248, 138, 91, 151, 118], OperandSize::Dword)
}

#[test]
fn vpsrlvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 133914099, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 69, 20, 189, 243, 93, 251, 7], OperandSize::Dword)
}

#[test]
fn vpsrlvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 5, 195, 69, 206], OperandSize::Qword)
}

#[test]
fn vpsrlvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 114627951, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 13, 206, 69, 132, 91, 111, 21, 213, 6], OperandSize::Qword)
}

#[test]
fn vpsrlvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 13, 211, 69, 28, 240], OperandSize::Qword)
}

