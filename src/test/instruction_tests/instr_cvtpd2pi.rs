use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 254], OperandSize::Word)
}

#[test]
fn cvtpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 35], OperandSize::Word)
}

#[test]
fn cvtpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 247], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 42], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 216], OperandSize::Qword)
}

#[test]
fn cvtpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 41534421, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 52, 149, 213, 195, 121, 2], OperandSize::Qword)
}

