use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 87, 205], OperandSize::Dword)
}

#[test]
fn vxorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 342001068, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 87, 168, 172, 133, 98, 20], OperandSize::Dword)
}

#[test]
fn vxorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 87, 221], OperandSize::Qword)
}

#[test]
fn vxorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 427393915, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 87, 155, 123, 131, 121, 25], OperandSize::Qword)
}

#[test]
fn vxorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 202], OperandSize::Dword)
}

#[test]
fn vxorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 87, 60, 199], OperandSize::Dword)
}

#[test]
fn vxorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 87, 240], OperandSize::Qword)
}

#[test]
fn vxorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 1663018221, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 87, 161, 237, 164, 31, 99], OperandSize::Qword)
}

#[test]
fn vxorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 76, 143, 87, 234], OperandSize::Dword)
}

#[test]
fn vxorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1765507242, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 138, 87, 146, 170, 128, 59, 105], OperandSize::Dword)
}

#[test]
fn vxorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1065157489, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 157, 87, 148, 86, 113, 3, 125, 63], OperandSize::Dword)
}

#[test]
fn vxorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 44, 135, 87, 250], OperandSize::Qword)
}

#[test]
fn vxorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 625285571, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 20, 132, 87, 60, 189, 195, 25, 69, 37], OperandSize::Qword)
}

#[test]
fn vxorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RDI, 184438238, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 60, 145, 87, 191, 222, 77, 254, 10], OperandSize::Qword)
}

#[test]
fn vxorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 175, 87, 236], OperandSize::Dword)
}

#[test]
fn vxorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 786049313, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 87, 28, 133, 33, 41, 218, 46], OperandSize::Dword)
}

#[test]
fn vxorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 188, 87, 10], OperandSize::Dword)
}

#[test]
fn vxorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 36, 170, 87, 196], OperandSize::Qword)
}

#[test]
fn vxorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1312204911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 100, 173, 87, 4, 245, 111, 168, 54, 78], OperandSize::Qword)
}

#[test]
fn vxorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 108, 178, 87, 60, 80], OperandSize::Qword)
}

#[test]
fn vxorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 203, 87, 231], OperandSize::Dword)
}

#[test]
fn vxorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 2099711656, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 205, 87, 143, 168, 14, 39, 125], OperandSize::Dword)
}

#[test]
fn vxorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 324239684, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 217, 87, 152, 68, 129, 83, 19], OperandSize::Dword)
}

#[test]
fn vxorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 44, 206, 87, 203], OperandSize::Qword)
}

#[test]
fn vxorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 884547819, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 68, 193, 87, 28, 205, 235, 32, 185, 52], OperandSize::Qword)
}

#[test]
fn vxorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDX, 1509061363, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 211, 87, 162, 243, 114, 242, 89], OperandSize::Qword)
}

