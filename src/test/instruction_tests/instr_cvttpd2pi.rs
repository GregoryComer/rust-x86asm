use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 228], OperandSize::Word)
}

#[test]
fn cvttpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 58], OperandSize::Word)
}

#[test]
fn cvttpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 239], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EAX, 399480528, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 184, 208, 150, 207, 23], OperandSize::Dword)
}

#[test]
fn cvttpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 226], OperandSize::Qword)
}

#[test]
fn cvttpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 44, 17], OperandSize::Qword)
}

