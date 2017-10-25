use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 140, 114, 200, 6], OperandSize::Dword)
}

#[test]
fn vprolq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 114, 12, 113, 94], OperandSize::Dword)
}

#[test]
fn vprolq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 793068157, Some(OperandSize::Qword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 158, 114, 12, 253, 125, 66, 69, 47, 81], OperandSize::Dword)
}

#[test]
fn vprolq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 114, 204, 126], OperandSize::Qword)
}

#[test]
fn vprolq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1005920133, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 114, 12, 93, 133, 31, 245, 59, 59], OperandSize::Qword)
}

#[test]
fn vprolq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1250071338, Some(OperandSize::Qword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 151, 114, 12, 125, 42, 147, 130, 74, 74], OperandSize::Qword)
}

#[test]
fn vprolq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 169, 114, 204, 80], OperandSize::Dword)
}

#[test]
fn vprolq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 956801328, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 114, 140, 159, 48, 161, 7, 57, 72], OperandSize::Dword)
}

#[test]
fn vprolq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 186, 114, 12, 87, 127], OperandSize::Dword)
}

#[test]
fn vprolq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 114, 206, 123], OperandSize::Qword)
}

#[test]
fn vprolq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 769249826, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 163, 114, 12, 69, 34, 210, 217, 45, 72], OperandSize::Qword)
}

#[test]
fn vprolq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 186, 114, 12, 207, 32], OperandSize::Qword)
}

#[test]
fn vprolq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 202, 114, 201, 6], OperandSize::Dword)
}

#[test]
fn vprolq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1409486135, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 205, 114, 12, 157, 55, 13, 3, 84, 106], OperandSize::Dword)
}

#[test]
fn vprolq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 922076723, Some(OperandSize::Qword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 114, 140, 86, 51, 198, 245, 54, 0], OperandSize::Dword)
}

#[test]
fn vprolq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 205, 194, 114, 202, 16], OperandSize::Qword)
}

#[test]
fn vprolq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(RBX, 965217842, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 114, 139, 50, 14, 136, 57, 37], OperandSize::Qword)
}

#[test]
fn vprolq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectDisplaced(RDI, 655145521, Some(OperandSize::Qword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 212, 114, 143, 49, 186, 12, 39, 8], OperandSize::Qword)
}

