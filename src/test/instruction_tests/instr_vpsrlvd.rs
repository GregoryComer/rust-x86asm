use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 69, 228], OperandSize::Dword)
}

#[test]
fn vpsrlvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 562434077, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 69, 174, 29, 16, 134, 33], OperandSize::Dword)
}

#[test]
fn vpsrlvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 69, 213], OperandSize::Qword)
}

#[test]
fn vpsrlvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1847119597, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 69, 28, 197, 237, 206, 24, 110], OperandSize::Qword)
}

#[test]
fn vpsrlvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 69, 193], OperandSize::Dword)
}

#[test]
fn vpsrlvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 69, 6], OperandSize::Dword)
}

#[test]
fn vpsrlvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 69, 225], OperandSize::Qword)
}

#[test]
fn vpsrlvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 727377897, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 69, 180, 159, 233, 231, 90, 43], OperandSize::Qword)
}

#[test]
fn vpsrlvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 69, 199], OperandSize::Dword)
}

#[test]
fn vpsrlvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 69, 50], OperandSize::Dword)
}

#[test]
fn vpsrlvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1437245062, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 69, 20, 69, 134, 158, 170, 85], OperandSize::Dword)
}

#[test]
fn vpsrlvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 53, 139, 69, 207], OperandSize::Qword)
}

#[test]
fn vpsrlvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 93, 139, 69, 20, 216], OperandSize::Qword)
}

#[test]
fn vpsrlvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 758079933, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 151, 69, 156, 88, 189, 97, 47, 45], OperandSize::Qword)
}

#[test]
fn vpsrlvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 69, 205], OperandSize::Dword)
}

#[test]
fn vpsrlvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1441455672, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 69, 60, 181, 56, 222, 234, 85], OperandSize::Dword)
}

#[test]
fn vpsrlvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1670665806, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 69, 188, 216, 78, 86, 148, 99], OperandSize::Dword)
}

#[test]
fn vpsrlvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 13, 163, 69, 247], OperandSize::Qword)
}

#[test]
fn vpsrlvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RAX, 1166314797, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 45, 172, 69, 152, 45, 141, 132, 69], OperandSize::Qword)
}

#[test]
fn vpsrlvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 397786860, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 69, 181, 69, 132, 186, 236, 190, 181, 23], OperandSize::Qword)
}

#[test]
fn vpsrlvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 69, 244], OperandSize::Dword)
}

#[test]
fn vpsrlvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 69, 34], OperandSize::Dword)
}

#[test]
fn vpsrlvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1066109031, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 69, 180, 159, 103, 136, 139, 63], OperandSize::Dword)
}

#[test]
fn vpsrlvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 77, 202, 69, 244], OperandSize::Qword)
}

#[test]
fn vpsrlvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RBX, 171032202, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 109, 199, 69, 163, 138, 190, 49, 10], OperandSize::Qword)
}

#[test]
fn vpsrlvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 601348576, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 210, 69, 164, 127, 224, 217, 215, 35], OperandSize::Qword)
}

