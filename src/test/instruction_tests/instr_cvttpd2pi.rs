use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 201], OperandSize::Word)
}

#[test]
fn cvttpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 15817, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 147, 201, 61], OperandSize::Word)
}

#[test]
fn cvttpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 203], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(EDI, 91030491, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 175, 219, 3, 109, 5], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 210], OperandSize::Qword)
}

#[test]
fn cvttpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(RSI, 627313718, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 158, 54, 12, 100, 37], OperandSize::Qword)
}

