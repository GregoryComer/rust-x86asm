use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1251278194, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 140, 60, 197, 114, 253, 148, 74], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 140, 18], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 140, 20, 146], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDX, 496810979, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 140, 178, 227, 187, 156, 29], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1277420393, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 142, 172, 113, 105, 227, 35, 76], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 500672394, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 142, 148, 70, 138, 167, 215, 29], OperandSize::Qword)
}

#[test]
fn vpmaskmovq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 746170752, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 142, 172, 135, 128, 169, 121, 44], OperandSize::Dword)
}

#[test]
fn vpmaskmovq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVQ, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1702642437, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 142, 164, 65, 5, 67, 124, 101], OperandSize::Qword)
}

