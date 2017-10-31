use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 239], OperandSize::Dword)
}

#[test]
fn pabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 34], OperandSize::Dword)
}

#[test]
fn pabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 231], OperandSize::Qword)
}

#[test]
fn pabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RCX, 377648716, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 177, 76, 118, 130, 22], OperandSize::Qword)
}

#[test]
fn pabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 210], OperandSize::Dword)
}

#[test]
fn pabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 60, 71], OperandSize::Dword)
}

#[test]
fn pabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 221], OperandSize::Qword)
}

#[test]
fn pabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1307999520, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 188, 126, 32, 125, 246, 77], OperandSize::Qword)
}

