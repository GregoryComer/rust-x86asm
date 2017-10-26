use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 59, 207], OperandSize::Dword)
}

#[test]
fn vpminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1778593041, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 59, 130, 17, 45, 3, 106], OperandSize::Dword)
}

#[test]
fn vpminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 59, 220], OperandSize::Qword)
}

#[test]
fn vpminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2096059145, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 59, 44, 125, 9, 83, 239, 124], OperandSize::Qword)
}

#[test]
fn vpminud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 59, 238], OperandSize::Dword)
}

#[test]
fn vpminud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 59, 33], OperandSize::Dword)
}

#[test]
fn vpminud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 59, 223], OperandSize::Qword)
}

#[test]
fn vpminud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1711359592, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 59, 52, 253, 104, 70, 1, 102], OperandSize::Qword)
}

#[test]
fn vpminud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 59, 204], OperandSize::Dword)
}

#[test]
fn vpminud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1747341982, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 59, 162, 158, 82, 38, 104], OperandSize::Dword)
}

#[test]
fn vpminud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2007502612, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 59, 60, 69, 20, 15, 168, 119], OperandSize::Dword)
}

#[test]
fn vpminud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 77, 134, 59, 195], OperandSize::Qword)
}

#[test]
fn vpminud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 117, 143, 59, 0], OperandSize::Qword)
}

#[test]
fn vpminud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 922111948, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 61, 150, 59, 148, 143, 204, 79, 246, 54], OperandSize::Qword)
}

#[test]
fn vpminud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 59, 235], OperandSize::Dword)
}

#[test]
fn vpminud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 601437156, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 59, 176, 228, 51, 217, 35], OperandSize::Dword)
}

#[test]
fn vpminud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 265812939, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 185, 59, 156, 142, 203, 251, 215, 15], OperandSize::Dword)
}

#[test]
fn vpminud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 166, 59, 225], OperandSize::Qword)
}

#[test]
fn vpminud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1596057095, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 37, 162, 59, 188, 143, 7, 230, 33, 95], OperandSize::Qword)
}

#[test]
fn vpminud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 77, 190, 59, 59], OperandSize::Qword)
}

#[test]
fn vpminud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 59, 203], OperandSize::Dword)
}

#[test]
fn vpminud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 59, 12, 130], OperandSize::Dword)
}

#[test]
fn vpminud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 59, 36, 151], OperandSize::Dword)
}

#[test]
fn vpminud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 117, 203, 59, 246], OperandSize::Qword)
}

#[test]
fn vpminud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 29, 196, 59, 17], OperandSize::Qword)
}

#[test]
fn vpminud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 93, 211, 59, 12, 158], OperandSize::Qword)
}

