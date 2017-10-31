use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 209], OperandSize::Dword)
}

#[test]
fn psubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 680038585, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 164, 154, 185, 144, 136, 40], OperandSize::Dword)
}

#[test]
fn psubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 245], OperandSize::Qword)
}

#[test]
fn psubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RSI, 596005632, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 232, 190, 0, 83, 134, 35], OperandSize::Qword)
}

#[test]
fn psubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 222], OperandSize::Dword)
}

#[test]
fn psubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 0], OperandSize::Dword)
}

#[test]
fn psubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 237], OperandSize::Qword)
}

#[test]
fn psubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 232, 36, 67], OperandSize::Qword)
}

