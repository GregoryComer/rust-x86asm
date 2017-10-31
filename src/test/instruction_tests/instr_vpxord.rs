use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 239, 233], OperandSize::Dword)
}

#[test]
fn vpxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 922641286, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 239, 44, 181, 134, 99, 254, 54], OperandSize::Dword)
}

#[test]
fn vpxord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 158, 239, 44, 95], OperandSize::Dword)
}

#[test]
fn vpxord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 93, 130, 239, 241], OperandSize::Qword)
}

#[test]
fn vpxord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 656762800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 101, 140, 239, 163, 176, 103, 37, 39], OperandSize::Qword)
}

#[test]
fn vpxord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 69, 145, 239, 52, 136], OperandSize::Qword)
}

#[test]
fn vpxord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 239, 204], OperandSize::Dword)
}

#[test]
fn vpxord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 2029262893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 171, 239, 28, 213, 45, 24, 244, 120], OperandSize::Dword)
}

#[test]
fn vpxord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 627569977, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 186, 239, 52, 141, 57, 245, 103, 37], OperandSize::Dword)
}

#[test]
fn vpxord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 53, 162, 239, 241], OperandSize::Qword)
}

#[test]
fn vpxord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 37, 173, 239, 31], OperandSize::Qword)
}

#[test]
fn vpxord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 101, 189, 239, 43], OperandSize::Qword)
}

#[test]
fn vpxord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 239, 234], OperandSize::Dword)
}

#[test]
fn vpxord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 239, 44, 241], OperandSize::Dword)
}

#[test]
fn vpxord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 519088584, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 220, 239, 154, 200, 169, 240, 30], OperandSize::Dword)
}

#[test]
fn vpxord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 21, 194, 239, 213], OperandSize::Qword)
}

#[test]
fn vpxord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RDI, 975420761, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 85, 206, 239, 135, 89, 189, 35, 58], OperandSize::Qword)
}

#[test]
fn vpxord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RBX, 215968735, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 37, 210, 239, 179, 223, 107, 223, 12], OperandSize::Qword)
}

