use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpd2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 221], OperandSize::Word)
}

#[test]
fn cvtpd2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 159, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 169, 159, 0], OperandSize::Word)
}

#[test]
fn cvtpd2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 205], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1488138821, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 36, 189, 69, 50, 179, 88], OperandSize::Dword)
}

#[test]
fn cvtpd2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 211], OperandSize::Qword)
}

#[test]
fn cvtpd2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPD2PI, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 45, 28, 185], OperandSize::Qword)
}

