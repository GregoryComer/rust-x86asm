use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 138, 239, 204], OperandSize::Dword)
}

#[test]
fn vpxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1555886485, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 139, 239, 140, 120, 149, 241, 188, 92], OperandSize::Dword)
}

#[test]
fn vpxorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 159, 239, 38], OperandSize::Dword)
}

#[test]
fn vpxorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 237, 133, 239, 221], OperandSize::Qword)
}

#[test]
fn vpxorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RDX, 1355460407, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 189, 141, 239, 138, 55, 175, 202, 80], OperandSize::Qword)
}

#[test]
fn vpxorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 149, 153, 239, 0], OperandSize::Qword)
}

#[test]
fn vpxorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 170, 239, 216], OperandSize::Dword)
}

#[test]
fn vpxorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 2069310183, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 239, 52, 197, 231, 42, 87, 123], OperandSize::Dword)
}

#[test]
fn vpxorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1018750814, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 190, 239, 52, 125, 94, 231, 184, 60], OperandSize::Dword)
}

#[test]
fn vpxorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 165, 167, 239, 214], OperandSize::Qword)
}

#[test]
fn vpxorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 141, 171, 239, 35], OperandSize::Qword)
}

#[test]
fn vpxorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 628616456, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 165, 186, 239, 60, 253, 8, 237, 119, 37], OperandSize::Qword)
}

#[test]
fn vpxorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 239, 223], OperandSize::Dword)
}

#[test]
fn vpxorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 61039457, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 239, 175, 97, 99, 163, 3], OperandSize::Dword)
}

#[test]
fn vpxorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 218, 239, 28, 216], OperandSize::Dword)
}

#[test]
fn vpxorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 221, 198, 239, 224], OperandSize::Qword)
}

#[test]
fn vpxorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RCX, 1265511470, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 205, 239, 161, 46, 44, 110, 75], OperandSize::Qword)
}

#[test]
fn vpxorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 229, 218, 239, 32], OperandSize::Qword)
}

