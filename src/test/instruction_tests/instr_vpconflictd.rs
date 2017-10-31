use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 196, 218], OperandSize::Dword)
}

#[test]
fn vpconflictd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 196, 60, 89], OperandSize::Dword)
}

#[test]
fn vpconflictd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 157, 196, 47], OperandSize::Dword)
}

#[test]
fn vpconflictd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 143, 196, 246], OperandSize::Qword)
}

#[test]
fn vpconflictd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RDI, 496741206, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 196, 159, 86, 171, 155, 29], OperandSize::Qword)
}

#[test]
fn vpconflictd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 179035349, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 196, 20, 157, 213, 220, 171, 10], OperandSize::Qword)
}

#[test]
fn vpconflictd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 196, 196], OperandSize::Dword)
}

#[test]
fn vpconflictd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 1143327580, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 196, 163, 92, 203, 37, 68], OperandSize::Dword)
}

#[test]
fn vpconflictd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 196, 31], OperandSize::Dword)
}

#[test]
fn vpconflictd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 170, 196, 250], OperandSize::Qword)
}

#[test]
fn vpconflictd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 409406898, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 169, 196, 180, 139, 178, 13, 103, 24], OperandSize::Qword)
}

#[test]
fn vpconflictd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1970806041, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 190, 196, 148, 91, 25, 29, 120, 117], OperandSize::Qword)
}

#[test]
fn vpconflictd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 196, 214], OperandSize::Dword)
}

#[test]
fn vpconflictd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 2075664339, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 196, 164, 183, 211, 31, 184, 123], OperandSize::Dword)
}

#[test]
fn vpconflictd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 402227019, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 196, 148, 186, 75, 127, 249, 23], OperandSize::Dword)
}

#[test]
fn vpconflictd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 196, 222], OperandSize::Qword)
}

#[test]
fn vpconflictd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 196, 12, 139], OperandSize::Qword)
}

#[test]
fn vpconflictd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RBX, 1919004110, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 221, 196, 179, 206, 173, 97, 114], OperandSize::Qword)
}

