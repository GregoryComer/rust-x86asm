use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 196, 205], OperandSize::Dword)
}

#[test]
fn vpconflictd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1563097415, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 196, 4, 85, 71, 249, 42, 93], OperandSize::Dword)
}

#[test]
fn vpconflictd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1099802957, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 196, 164, 211, 77, 169, 141, 65], OperandSize::Dword)
}

#[test]
fn vpconflictd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 196, 196], OperandSize::Qword)
}

#[test]
fn vpconflictd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectDisplaced(RDX, 701109630, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 196, 186, 126, 21, 202, 41], OperandSize::Qword)
}

#[test]
fn vpconflictd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1588524358, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 156, 196, 36, 213, 70, 245, 174, 94], OperandSize::Qword)
}

#[test]
fn vpconflictd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 196, 203], OperandSize::Dword)
}

#[test]
fn vpconflictd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 1707093808, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 196, 153, 48, 47, 192, 101], OperandSize::Dword)
}

#[test]
fn vpconflictd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EAX, 1861045969, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 196, 136, 209, 78, 237, 110], OperandSize::Dword)
}

#[test]
fn vpconflictd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 169, 196, 250], OperandSize::Qword)
}

#[test]
fn vpconflictd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 256819801, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 196, 188, 64, 89, 194, 78, 15], OperandSize::Qword)
}

#[test]
fn vpconflictd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1476983542, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 196, 148, 242, 246, 250, 8, 88], OperandSize::Qword)
}

#[test]
fn vpconflictd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 196, 218], OperandSize::Dword)
}

#[test]
fn vpconflictd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 196, 16], OperandSize::Dword)
}

#[test]
fn vpconflictd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 196, 36, 134], OperandSize::Dword)
}

#[test]
fn vpconflictd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 202, 196, 234], OperandSize::Qword)
}

#[test]
fn vpconflictd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 196, 28, 89], OperandSize::Qword)
}

#[test]
fn vpconflictd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1829688548, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 196, 12, 213, 228, 212, 14, 109], OperandSize::Qword)
}

