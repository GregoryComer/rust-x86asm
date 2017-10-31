use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 59, 218], OperandSize::Dword)
}

#[test]
fn vpminuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 59, 40], OperandSize::Dword)
}

#[test]
fn vpminuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 59, 60, 217], OperandSize::Dword)
}

#[test]
fn vpminuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 213, 131, 59, 213], OperandSize::Qword)
}

#[test]
fn vpminuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 2004670663, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 149, 134, 59, 148, 71, 199, 216, 124, 119], OperandSize::Qword)
}

#[test]
fn vpminuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 145, 59, 19], OperandSize::Qword)
}

#[test]
fn vpminuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 59, 238], OperandSize::Dword)
}

#[test]
fn vpminuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 596979530, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 59, 175, 74, 47, 149, 35], OperandSize::Dword)
}

#[test]
fn vpminuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 186, 59, 47], OperandSize::Dword)
}

#[test]
fn vpminuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 245, 174, 59, 218], OperandSize::Qword)
}

#[test]
fn vpminuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1023733146, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 149, 175, 59, 28, 189, 154, 237, 4, 61], OperandSize::Qword)
}

#[test]
fn vpminuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 178, 59, 33], OperandSize::Qword)
}

#[test]
fn vpminuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 59, 242], OperandSize::Dword)
}

#[test]
fn vpminuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1969131107, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 59, 140, 138, 99, 142, 94, 117], OperandSize::Dword)
}

#[test]
fn vpminuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1508271311, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 59, 156, 135, 207, 100, 230, 89], OperandSize::Dword)
}

#[test]
fn vpminuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 213, 198, 59, 234], OperandSize::Qword)
}

#[test]
fn vpminuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 193, 59, 40], OperandSize::Qword)
}

#[test]
fn vpminuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 189, 215, 59, 12, 203], OperandSize::Qword)
}

