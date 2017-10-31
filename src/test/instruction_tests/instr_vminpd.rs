use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 93, 237], OperandSize::Dword)
}

#[test]
fn vminpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 93, 38], OperandSize::Dword)
}

#[test]
fn vminpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 93, 248], OperandSize::Qword)
}

#[test]
fn vminpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 61731572, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 93, 36, 221, 244, 242, 173, 3], OperandSize::Qword)
}

#[test]
fn vminpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 93, 248], OperandSize::Dword)
}

#[test]
fn vminpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 93, 36, 154], OperandSize::Dword)
}

#[test]
fn vminpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 93, 255], OperandSize::Qword)
}

#[test]
fn vminpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1907265040, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 93, 172, 137, 16, 142, 174, 113], OperandSize::Qword)
}

#[test]
fn vminpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 93, 212], OperandSize::Dword)
}

#[test]
fn vminpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 966627465, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 138, 93, 183, 137, 144, 157, 57], OperandSize::Dword)
}

#[test]
fn vminpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 153, 93, 43], OperandSize::Dword)
}

#[test]
fn vminpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 245, 140, 93, 255], OperandSize::Qword)
}

#[test]
fn vminpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1229069071, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 157, 129, 93, 148, 209, 15, 27, 66, 73], OperandSize::Qword)
}

#[test]
fn vminpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 116183002, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 173, 157, 93, 148, 209, 218, 207, 236, 6], OperandSize::Qword)
}

#[test]
fn vminpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 171, 93, 218], OperandSize::Dword)
}

#[test]
fn vminpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 170, 93, 60, 192], OperandSize::Dword)
}

#[test]
fn vminpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1183177812, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 186, 93, 20, 117, 84, 220, 133, 70], OperandSize::Dword)
}

#[test]
fn vminpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 229, 166, 93, 209], OperandSize::Qword)
}

#[test]
fn vminpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 165, 175, 93, 12, 147], OperandSize::Qword)
}

#[test]
fn vminpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 237, 178, 93, 44, 144], OperandSize::Qword)
}

#[test]
fn vminpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 155, 93, 241], OperandSize::Dword)
}

#[test]
fn vminpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 203, 93, 63], OperandSize::Dword)
}

#[test]
fn vminpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1740161734, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 220, 93, 187, 198, 194, 184, 103], OperandSize::Dword)
}

#[test]
fn vminpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 173, 154, 93, 219], OperandSize::Qword)
}

#[test]
fn vminpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RAX, 1876286313, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 245, 194, 93, 184, 105, 219, 213, 111], OperandSize::Qword)
}

#[test]
fn vminpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 252075671, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 222, 93, 164, 115, 151, 94, 6, 15], OperandSize::Qword)
}

