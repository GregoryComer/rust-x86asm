use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 93, 245], OperandSize::Dword)
}

#[test]
fn vminpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 808706332, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 93, 4, 117, 28, 225, 51, 48], OperandSize::Dword)
}

#[test]
fn vminpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 93, 230], OperandSize::Qword)
}

#[test]
fn vminpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1886594825, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 93, 188, 67, 9, 39, 115, 112], OperandSize::Qword)
}

#[test]
fn vminpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 93, 255], OperandSize::Dword)
}

#[test]
fn vminpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1532542842, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 93, 4, 197, 122, 191, 88, 91], OperandSize::Dword)
}

#[test]
fn vminpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 93, 221], OperandSize::Qword)
}

#[test]
fn vminpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1311683577, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 93, 156, 75, 249, 179, 46, 78], OperandSize::Qword)
}

#[test]
fn vminpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 138, 93, 239], OperandSize::Dword)
}

#[test]
fn vminpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1360800118, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 93, 60, 245, 118, 41, 28, 81], OperandSize::Dword)
}

#[test]
fn vminpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 88305933, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 158, 93, 20, 213, 13, 113, 67, 5], OperandSize::Dword)
}

#[test]
fn vminpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 253, 143, 93, 248], OperandSize::Qword)
}

#[test]
fn vminpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 142, 93, 28, 183], OperandSize::Qword)
}

#[test]
fn vminpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 443290686, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 181, 157, 93, 164, 119, 62, 20, 108, 26], OperandSize::Qword)
}

#[test]
fn vminpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 175, 93, 247], OperandSize::Dword)
}

#[test]
fn vminpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 174, 93, 49], OperandSize::Dword)
}

#[test]
fn vminpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 189, 93, 43], OperandSize::Dword)
}

#[test]
fn vminpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 93, 220], OperandSize::Qword)
}

#[test]
fn vminpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 161, 93, 4, 177], OperandSize::Qword)
}

#[test]
fn vminpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1384155281, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 179, 93, 180, 90, 145, 136, 128, 82], OperandSize::Qword)
}

#[test]
fn vminpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 155, 93, 212], OperandSize::Dword)
}

#[test]
fn vminpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 945942180, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 201, 93, 36, 253, 164, 238, 97, 56], OperandSize::Dword)
}

#[test]
fn vminpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 219, 93, 52, 193], OperandSize::Dword)
}

#[test]
fn vminpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 205, 159, 93, 243], OperandSize::Qword)
}

#[test]
fn vminpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 141, 193, 93, 24], OperandSize::Qword)
}

#[test]
fn vminpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1733656194, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 165, 212, 93, 36, 189, 130, 126, 85, 103], OperandSize::Qword)
}

