use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 196, 236], OperandSize::Dword)
}

#[test]
fn vpconflictd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 1885256746, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 196, 139, 42, 188, 94, 112], OperandSize::Dword)
}

#[test]
fn vpconflictd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 293394690, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 196, 146, 2, 217, 124, 17], OperandSize::Dword)
}

#[test]
fn vpconflictd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 125, 137, 196, 247], OperandSize::Qword)
}

#[test]
fn vpconflictd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 196, 60, 89], OperandSize::Qword)
}

#[test]
fn vpconflictd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 156, 196, 44, 194], OperandSize::Qword)
}

#[test]
fn vpconflictd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 196, 206], OperandSize::Dword)
}

#[test]
fn vpconflictd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 196, 42], OperandSize::Dword)
}

#[test]
fn vpconflictd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 196, 20, 246], OperandSize::Dword)
}

#[test]
fn vpconflictd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 172, 196, 244], OperandSize::Qword)
}

#[test]
fn vpconflictd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 196, 43], OperandSize::Qword)
}

#[test]
fn vpconflictd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 472553893, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 191, 196, 180, 217, 165, 153, 42, 28], OperandSize::Qword)
}

#[test]
fn vpconflictd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 196, 206], OperandSize::Dword)
}

#[test]
fn vpconflictd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 196, 38], OperandSize::Dword)
}

#[test]
fn vpconflictd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 196, 32], OperandSize::Dword)
}

#[test]
fn vpconflictd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 125, 202, 196, 226], OperandSize::Qword)
}

#[test]
fn vpconflictd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 994066103, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 196, 148, 150, 183, 62, 64, 59], OperandSize::Qword)
}

#[test]
fn vpconflictd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM19)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 223, 196, 25], OperandSize::Qword)
}

