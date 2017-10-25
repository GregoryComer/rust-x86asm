use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 201], OperandSize::Dword)
}

#[test]
fn aesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1313641433, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 180, 240, 217, 147, 76, 78], OperandSize::Dword)
}

#[test]
fn aesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 236], OperandSize::Qword)
}

#[test]
fn aesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 18], OperandSize::Qword)
}

