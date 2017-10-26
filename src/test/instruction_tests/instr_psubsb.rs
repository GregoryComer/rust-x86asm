use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 237], OperandSize::Dword)
}

#[test]
fn psubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 4, 208], OperandSize::Dword)
}

#[test]
fn psubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 236], OperandSize::Qword)
}

#[test]
fn psubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 12, 86], OperandSize::Qword)
}

#[test]
fn psubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 246], OperandSize::Dword)
}

#[test]
fn psubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 32], OperandSize::Dword)
}

#[test]
fn psubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 233], OperandSize::Qword)
}

#[test]
fn psubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 549130752, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 164, 78, 0, 18, 187, 32], OperandSize::Qword)
}

