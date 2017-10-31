use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 139, 240], OperandSize::Dword)
}

#[test]
fn vpcompressq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(ESI, Two, 584775421, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 139, 60, 117, 253, 246, 218, 34], OperandSize::Dword)
}

#[test]
fn vpcompressq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 139, 139, 239], OperandSize::Qword)
}

#[test]
fn vpcompressq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 139, 26], OperandSize::Qword)
}

#[test]
fn vpcompressq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 139, 205], OperandSize::Dword)
}

#[test]
fn vpcompressq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(EDI, Four, 87913793, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 139, 4, 189, 65, 117, 61, 5], OperandSize::Dword)
}

#[test]
fn vpcompressq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 253, 175, 139, 221], OperandSize::Qword)
}

#[test]
fn vpcompressq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 472151251, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 40, 139, 4, 205, 211, 116, 36, 28], OperandSize::Qword)
}

#[test]
fn vpcompressq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 139, 202], OperandSize::Dword)
}

#[test]
fn vpcompressq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 36, 218], OperandSize::Dword)
}

#[test]
fn vpcompressq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 253, 203, 139, 220], OperandSize::Qword)
}

#[test]
fn vpcompressq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 2041324131, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 72, 139, 156, 218, 99, 34, 172, 121], OperandSize::Qword)
}

