use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 190, 225], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 239011697, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 190, 156, 67, 113, 7, 63, 14], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 190, 233], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 788543611, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 190, 167, 123, 56, 0, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 190, 221], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 650613610, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 190, 147, 106, 147, 199, 38], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 190, 235], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1105814396, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 60, 77, 124, 99, 233, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 190, 221], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 1555310584, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 190, 184, 248, 39, 180, 92], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 1329917085, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 157, 190, 151, 157, 236, 68, 79], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 221, 142, 190, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 492361080, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 229, 137, 190, 148, 214, 120, 213, 88, 29], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 197, 155, 190, 31], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 190, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 1480222513, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 190, 152, 49, 103, 58, 88], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1387245022, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 186, 190, 140, 206, 222, 173, 175, 82], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 189, 175, 190, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RCX, 802956741, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 172, 190, 177, 197, 37, 220, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 157, 183, 190, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 218, 190, 230], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ESI, 627294192, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 190, 150, 240, 191, 99, 37], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1701862274, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 190, 156, 145, 130, 91, 112, 101], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 133, 244, 190, 251], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RBX, 814915369, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 221, 195, 190, 187, 41, 159, 146, 48], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1337530060, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 220, 190, 140, 70, 204, 22, 185, 79], OperandSize::Qword)
}

