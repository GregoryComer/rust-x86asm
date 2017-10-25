use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 186, 194], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 1467932140, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 186, 186, 236, 221, 126, 87], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 186, 245], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 186, 27], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 186, 244], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 186, 39], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 186, 208], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2039312500, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 186, 52, 93, 116, 112, 141, 121], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 186, 240], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 137, 186, 36, 142], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 448120940, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 158, 186, 134, 108, 200, 181, 26], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 213, 143, 186, 223], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 189, 138, 186, 26], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDI, 1205902633, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 146, 186, 183, 41, 157, 224, 71], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 186, 196], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 680433799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 186, 191, 135, 152, 142, 40], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 187, 186, 1], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 221, 170, 186, 255], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1596446833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 157, 172, 186, 4, 77, 113, 216, 39, 95], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RDX, 582162813, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 181, 186, 170, 125, 25, 179, 34], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 159, 186, 200], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 186, 28, 142], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 220, 186, 31], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 229, 157, 186, 212], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 173, 203, 186, 19], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RSI, 890584083, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 197, 209, 186, 134, 19, 60, 21, 53], OperandSize::Qword)
}

