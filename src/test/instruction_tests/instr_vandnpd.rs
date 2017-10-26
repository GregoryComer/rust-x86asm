use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 85, 236], OperandSize::Dword)
}

#[test]
fn vandnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1547368452, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 85, 188, 87, 4, 248, 58, 92], OperandSize::Dword)
}

#[test]
fn vandnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 85, 197], OperandSize::Qword)
}

#[test]
fn vandnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RSI, 1767843889, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 85, 166, 49, 40, 95, 105], OperandSize::Qword)
}

#[test]
fn vandnpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 85, 224], OperandSize::Dword)
}

#[test]
fn vandnpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1059555460, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 85, 12, 117, 132, 136, 39, 63], OperandSize::Dword)
}

#[test]
fn vandnpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 85, 240], OperandSize::Qword)
}

#[test]
fn vandnpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 85, 1], OperandSize::Qword)
}

#[test]
fn vandnpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 140, 85, 201], OperandSize::Dword)
}

#[test]
fn vandnpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 1119175325, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 85, 145, 157, 66, 181, 66], OperandSize::Dword)
}

#[test]
fn vandnpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 153, 85, 23], OperandSize::Dword)
}

#[test]
fn vandnpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 205, 140, 85, 211], OperandSize::Qword)
}

#[test]
fn vandnpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1982003484, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 221, 138, 85, 156, 246, 28, 249, 34, 118], OperandSize::Qword)
}

#[test]
fn vandnpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 245, 159, 85, 40], OperandSize::Qword)
}

#[test]
fn vandnpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 85, 242], OperandSize::Dword)
}

#[test]
fn vandnpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1261348881, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 85, 20, 77, 17, 168, 46, 75], OperandSize::Dword)
}

#[test]
fn vandnpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 186, 85, 4, 131], OperandSize::Dword)
}

#[test]
fn vandnpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 237, 162, 85, 203], OperandSize::Qword)
}

#[test]
fn vandnpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RAX, 986547420, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 161, 85, 128, 220, 132, 205, 58], OperandSize::Qword)
}

#[test]
fn vandnpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1237028825, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 183, 85, 140, 191, 217, 143, 187, 73], OperandSize::Qword)
}

#[test]
fn vandnpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 85, 219], OperandSize::Dword)
}

#[test]
fn vandnpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 231441547, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 85, 20, 245, 139, 132, 203, 13], OperandSize::Dword)
}

#[test]
fn vandnpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1948944364, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 221, 85, 164, 193, 236, 135, 42, 116], OperandSize::Dword)
}

#[test]
fn vandnpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 165, 199, 85, 192], OperandSize::Qword)
}

#[test]
fn vandnpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 75458769, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 165, 196, 85, 60, 245, 209, 104, 127, 4], OperandSize::Qword)
}

#[test]
fn vandnpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 157, 222, 85, 36, 247], OperandSize::Qword)
}

