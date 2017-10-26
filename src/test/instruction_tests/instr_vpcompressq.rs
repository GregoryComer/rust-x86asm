use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 139, 193], OperandSize::Dword)
}

#[test]
fn vpcompressq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1349892592, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 139, 180, 79, 240, 185, 117, 80], OperandSize::Dword)
}

#[test]
fn vpcompressq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 139, 239], OperandSize::Qword)
}

#[test]
fn vpcompressq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(RCX, Two, 722034298, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 139, 36, 77, 122, 94, 9, 43], OperandSize::Qword)
}

#[test]
fn vpcompressq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 139, 244], OperandSize::Dword)
}

#[test]
fn vpcompressq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 139, 22], OperandSize::Dword)
}

#[test]
fn vpcompressq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 175, 139, 243], OperandSize::Qword)
}

#[test]
fn vpcompressq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectDisplaced(RDX, 950270105, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 40, 139, 146, 153, 248, 163, 56], OperandSize::Qword)
}

#[test]
fn vpcompressq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 139, 200], OperandSize::Dword)
}

#[test]
fn vpcompressq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 44, 200], OperandSize::Dword)
}

#[test]
fn vpcompressq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 253, 202, 139, 241], OperandSize::Qword)
}

#[test]
fn vpcompressq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1195065597, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 72, 139, 148, 193, 253, 64, 59, 71], OperandSize::Qword)
}

