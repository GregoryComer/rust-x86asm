use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 193], OperandSize::Dword)
}

#[test]
fn psubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(EDX, 2105865668, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 162, 196, 245, 132, 125], OperandSize::Dword)
}

#[test]
fn psubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 243], OperandSize::Qword)
}

#[test]
fn psubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 60, 71], OperandSize::Qword)
}

#[test]
fn psubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 207], OperandSize::Dword)
}

#[test]
fn psubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 55], OperandSize::Dword)
}

#[test]
fn psubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 244], OperandSize::Qword)
}

#[test]
fn psubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 19], OperandSize::Qword)
}

