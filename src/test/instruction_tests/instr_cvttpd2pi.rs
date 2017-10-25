use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 255], OperandSize::Word)
}

#[test]
fn cvttpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(SI, 13, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 76, 13], OperandSize::Word)
}

#[test]
fn cvttpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 242], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(EAX, 1397018855, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 160, 231, 208, 68, 83], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 202], OperandSize::Qword)
}

#[test]
fn cvttpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 47], OperandSize::Qword)
}

