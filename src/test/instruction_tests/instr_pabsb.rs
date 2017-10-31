use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 200], OperandSize::Dword)
}

#[test]
fn pabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 661996639, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 186, 95, 68, 117, 39], OperandSize::Dword)
}

#[test]
fn pabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 193], OperandSize::Qword)
}

#[test]
fn pabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 120728465, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 188, 241, 145, 43, 50, 7], OperandSize::Qword)
}

#[test]
fn pabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 238], OperandSize::Dword)
}

#[test]
fn pabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 22], OperandSize::Dword)
}

#[test]
fn pabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 243], OperandSize::Qword)
}

#[test]
fn pabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 60, 200], OperandSize::Qword)
}

