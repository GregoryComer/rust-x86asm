use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 227], OperandSize::Word)
}

#[test]
fn cvttpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 10141, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 185, 157, 39], OperandSize::Word)
}

#[test]
fn cvttpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 210], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDI, 581734668, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 151, 12, 145, 172, 34], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 202], OperandSize::Qword)
}

#[test]
fn cvttpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 305752668, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 20, 205, 92, 106, 57, 18], OperandSize::Qword)
}

