use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 207], OperandSize::Dword)
}

#[test]
fn vptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 762397586, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 36, 205, 146, 67, 113, 45], OperandSize::Dword)
}

#[test]
fn vptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 255], OperandSize::Qword)
}

#[test]
fn vptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1952034191, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 180, 79, 143, 173, 89, 116], OperandSize::Qword)
}

#[test]
fn vptest_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 242], OperandSize::Dword)
}

#[test]
fn vptest_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDI, 1011782235, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 191, 91, 146, 78, 60], OperandSize::Dword)
}

#[test]
fn vptest_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 250], OperandSize::Qword)
}

#[test]
fn vptest_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 24], OperandSize::Qword)
}

