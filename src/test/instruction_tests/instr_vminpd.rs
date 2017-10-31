use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 93, 213], OperandSize::Dword)
}

#[test]
fn vminpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1981229872, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 93, 169, 48, 43, 23, 118], OperandSize::Dword)
}

#[test]
fn vminpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 93, 213], OperandSize::Qword)
}

#[test]
fn vminpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 220067334, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 93, 140, 186, 6, 246, 29, 13], OperandSize::Qword)
}

#[test]
fn vminpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 93, 239], OperandSize::Dword)
}

#[test]
fn vminpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 93, 63], OperandSize::Dword)
}

#[test]
fn vminpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 93, 231], OperandSize::Qword)
}

#[test]
fn vminpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 93, 20, 183], OperandSize::Qword)
}

#[test]
fn vminpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 141, 93, 234], OperandSize::Dword)
}

#[test]
fn vminpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 93, 4, 209], OperandSize::Dword)
}

#[test]
fn vminpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 932297188, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 154, 93, 36, 149, 228, 185, 145, 55], OperandSize::Dword)
}

#[test]
fn vminpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 134, 93, 217], OperandSize::Qword)
}

#[test]
fn vminpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1684101754, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 133, 93, 4, 85, 122, 90, 97, 100], OperandSize::Qword)
}

#[test]
fn vminpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1814942003, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 158, 93, 52, 245, 51, 209, 45, 108], OperandSize::Qword)
}

#[test]
fn vminpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 175, 93, 199], OperandSize::Dword)
}

#[test]
fn vminpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1513619059, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 93, 153, 115, 254, 55, 90], OperandSize::Dword)
}

#[test]
fn vminpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 1627522693, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 185, 93, 128, 133, 6, 2, 97], OperandSize::Dword)
}

#[test]
fn vminpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 229, 167, 93, 200], OperandSize::Qword)
}

#[test]
fn vminpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 462802832, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 197, 175, 93, 164, 71, 144, 207, 149, 27], OperandSize::Qword)
}

#[test]
fn vminpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RAX, 1999793638, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 189, 188, 93, 160, 230, 109, 50, 119], OperandSize::Qword)
}

#[test]
fn vminpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 158, 93, 218], OperandSize::Dword)
}

#[test]
fn vminpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 93, 44, 155], OperandSize::Dword)
}

#[test]
fn vminpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 877077007, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 222, 93, 44, 205, 15, 34, 71, 52], OperandSize::Dword)
}

#[test]
fn vminpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 197, 154, 93, 251], OperandSize::Qword)
}

#[test]
fn vminpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 229, 196, 93, 18], OperandSize::Qword)
}

#[test]
fn vminpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 799947276, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 213, 209, 93, 172, 250, 12, 58, 174, 47], OperandSize::Qword)
}

