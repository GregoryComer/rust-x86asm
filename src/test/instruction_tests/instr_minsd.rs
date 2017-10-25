use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 200], OperandSize::Dword)
}

#[test]
fn minsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 57], OperandSize::Dword)
}

#[test]
fn minsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 250], OperandSize::Qword)
}

#[test]
fn minsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 44, 202], OperandSize::Qword)
}

