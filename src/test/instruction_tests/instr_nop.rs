use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn nop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 198], OperandSize::Word)
}

#[test]
fn nop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 4524, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 129, 172, 17], OperandSize::Word)
}

#[test]
fn nop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 195], OperandSize::Dword)
}

#[test]
fn nop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(ECX, 1992377088, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 129, 0, 67, 193, 118], OperandSize::Dword)
}

#[test]
fn nop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 197], OperandSize::Qword)
}

#[test]
fn nop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 4, 218], OperandSize::Qword)
}

#[test]
fn nop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 199], OperandSize::Word)
}

#[test]
fn nop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 170, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 129, 170, 0], OperandSize::Word)
}

#[test]
fn nop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 193], OperandSize::Dword)
}

#[test]
fn nop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 4, 119], OperandSize::Dword)
}

#[test]
fn nop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 194], OperandSize::Qword)
}

#[test]
fn nop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(RAX, 578889765, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 128, 37, 40, 129, 34], OperandSize::Qword)
}

