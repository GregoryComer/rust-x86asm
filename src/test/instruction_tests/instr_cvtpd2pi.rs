use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 228], OperandSize::Word)
}

#[test]
fn cvtpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(Indirect(DI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 37], OperandSize::Word)
}

#[test]
fn cvtpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 196], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EBX, 1341974829, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 131, 45, 233, 252, 79], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 201], OperandSize::Qword)
}

#[test]
fn cvtpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RSI, 298995038, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 134, 94, 77, 210, 17], OperandSize::Qword)
}

