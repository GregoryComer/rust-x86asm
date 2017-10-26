use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn nop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 196], OperandSize::Word)
}

#[test]
fn nop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(BX, 186, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 135, 186, 0], OperandSize::Word)
}

#[test]
fn nop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 194], OperandSize::Dword)
}

#[test]
fn nop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 6], OperandSize::Dword)
}

#[test]
fn nop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 199], OperandSize::Qword)
}

#[test]
fn nop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 2], OperandSize::Qword)
}

#[test]
fn nop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 199], OperandSize::Word)
}

#[test]
fn nop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 242, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 131, 242, 0], OperandSize::Word)
}

#[test]
fn nop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 199], OperandSize::Dword)
}

#[test]
fn nop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 34296308, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 132, 70, 244, 81, 11, 2], OperandSize::Dword)
}

#[test]
fn nop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 193], OperandSize::Qword)
}

#[test]
fn nop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(RBX, 957760881, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 131, 113, 69, 22, 57], OperandSize::Qword)
}

