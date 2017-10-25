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
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 69, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 65, 69], OperandSize::Word)
}

#[test]
fn nop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 197], OperandSize::Dword)
}

#[test]
fn nop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1179195015, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 4, 77, 135, 22, 73, 70], OperandSize::Dword)
}

#[test]
fn nop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 195], OperandSize::Qword)
}

#[test]
fn nop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 4, 209], OperandSize::Qword)
}

#[test]
fn nop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 194], OperandSize::Word)
}

#[test]
fn nop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(SI, 15717, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 132, 101, 61], OperandSize::Word)
}

#[test]
fn nop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 193], OperandSize::Dword)
}

#[test]
fn nop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1057110044, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 132, 223, 28, 56, 2, 63], OperandSize::Dword)
}

#[test]
fn nop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 198], OperandSize::Qword)
}

#[test]
fn nop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1675962987, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 132, 127, 107, 42, 229, 99], OperandSize::Qword)
}

