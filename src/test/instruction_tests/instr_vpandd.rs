use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 219, 242], OperandSize::Dword)
}

#[test]
fn vpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1012102248, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 219, 44, 77, 104, 116, 83, 60], OperandSize::Dword)
}

#[test]
fn vpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 157, 219, 44, 135], OperandSize::Dword)
}

#[test]
fn vpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 13, 129, 219, 217], OperandSize::Qword)
}

#[test]
fn vpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1518626493, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 61, 131, 219, 20, 197, 189, 102, 132, 90], OperandSize::Qword)
}

#[test]
fn vpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 1213389433, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 93, 158, 219, 182, 121, 218, 82, 72], OperandSize::Qword)
}

#[test]
fn vpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 219, 198], OperandSize::Dword)
}

#[test]
fn vpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 172, 219, 28, 126], OperandSize::Dword)
}

#[test]
fn vpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 301168833, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 187, 219, 172, 202, 193, 120, 243, 17], OperandSize::Dword)
}

#[test]
fn vpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 77, 167, 219, 248], OperandSize::Qword)
}

#[test]
fn vpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 759386756, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 109, 173, 219, 161, 132, 82, 67, 45], OperandSize::Qword)
}

#[test]
fn vpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RBX, 444942118, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 69, 177, 219, 187, 38, 71, 133, 26], OperandSize::Qword)
}

#[test]
fn vpandd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 219, 209], OperandSize::Dword)
}

#[test]
fn vpandd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 690801279, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 203, 219, 186, 127, 202, 44, 41], OperandSize::Dword)
}

#[test]
fn vpandd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 220, 219, 28, 190], OperandSize::Dword)
}

#[test]
fn vpandd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 93, 203, 219, 245], OperandSize::Qword)
}

#[test]
fn vpandd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 293119512, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 69, 204, 219, 20, 221, 24, 166, 120, 17], OperandSize::Qword)
}

#[test]
fn vpandd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RDX, 1285668066, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 101, 219, 219, 170, 226, 188, 161, 76], OperandSize::Qword)
}

