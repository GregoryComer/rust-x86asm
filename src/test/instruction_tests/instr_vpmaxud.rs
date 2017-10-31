use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 63, 241], OperandSize::Dword)
}

#[test]
fn vpmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1087935934, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 63, 148, 152, 190, 149, 216, 64], OperandSize::Dword)
}

#[test]
fn vpmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 63, 240], OperandSize::Qword)
}

#[test]
fn vpmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 490271571, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 63, 148, 73, 83, 243, 56, 29], OperandSize::Qword)
}

#[test]
fn vpmaxud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 63, 220], OperandSize::Dword)
}

#[test]
fn vpmaxud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 783543335, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 63, 152, 39, 236, 179, 46], OperandSize::Dword)
}

#[test]
fn vpmaxud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 63, 226], OperandSize::Qword)
}

#[test]
fn vpmaxud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RAX, 678224504, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 63, 144, 120, 226, 108, 40], OperandSize::Qword)
}

#[test]
fn vpmaxud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 63, 196], OperandSize::Dword)
}

#[test]
fn vpmaxud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 1021134545, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 63, 184, 209, 70, 221, 60], OperandSize::Dword)
}

#[test]
fn vpmaxud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1206810230, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 158, 63, 12, 189, 118, 118, 238, 71], OperandSize::Dword)
}

#[test]
fn vpmaxud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 45, 131, 63, 236], OperandSize::Qword)
}

#[test]
fn vpmaxud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 69, 132, 63, 32], OperandSize::Qword)
}

#[test]
fn vpmaxud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 149, 63, 60, 118], OperandSize::Qword)
}

#[test]
fn vpmaxud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 63, 213], OperandSize::Dword)
}

#[test]
fn vpmaxud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 63, 40], OperandSize::Dword)
}

#[test]
fn vpmaxud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1985532002, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 188, 63, 52, 221, 98, 208, 88, 118], OperandSize::Dword)
}

#[test]
fn vpmaxud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 29, 161, 63, 239], OperandSize::Qword)
}

#[test]
fn vpmaxud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1246180244, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 63, 4, 197, 148, 51, 71, 74], OperandSize::Qword)
}

#[test]
fn vpmaxud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RBX, 1508640899, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 61, 186, 63, 163, 131, 8, 236, 89], OperandSize::Qword)
}

#[test]
fn vpmaxud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 63, 219], OperandSize::Dword)
}

#[test]
fn vpmaxud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 205, 63, 15], OperandSize::Dword)
}

#[test]
fn vpmaxud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 63, 22], OperandSize::Dword)
}

#[test]
fn vpmaxud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 101, 206, 63, 197], OperandSize::Qword)
}

#[test]
fn vpmaxud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 21, 196, 63, 10], OperandSize::Qword)
}

#[test]
fn vpmaxud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 217, 63, 48], OperandSize::Qword)
}

