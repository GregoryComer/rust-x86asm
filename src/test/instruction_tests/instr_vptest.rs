use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 194], OperandSize::Dword)
}

#[test]
fn vptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 3], OperandSize::Dword)
}

#[test]
fn vptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 210], OperandSize::Qword)
}

#[test]
fn vptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDX, 678661861, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 154, 229, 142, 115, 40], OperandSize::Qword)
}

#[test]
fn vptest_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 234], OperandSize::Dword)
}

#[test]
fn vptest_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDX, 1946696418, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 178, 226, 58, 8, 116], OperandSize::Dword)
}

#[test]
fn vptest_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 250], OperandSize::Qword)
}

#[test]
fn vptest_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 4, 242], OperandSize::Qword)
}

