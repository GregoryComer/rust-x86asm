use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn nop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 199], OperandSize::Word)
}

#[test]
fn nop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 126, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 66, 126], OperandSize::Word)
}

#[test]
fn nop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 197], OperandSize::Dword)
}

#[test]
fn nop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledDisplaced(ECX, Two, 930651337, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 4, 77, 201, 156, 120, 55], OperandSize::Dword)
}

#[test]
fn nop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 194], OperandSize::Qword)
}

#[test]
fn nop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledDisplaced(RAX, Two, 811161510, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 4, 69, 166, 87, 89, 48], OperandSize::Qword)
}

#[test]
fn nop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 197], OperandSize::Word)
}

#[test]
fn nop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(BX, 235, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 135, 235, 0], OperandSize::Word)
}

#[test]
fn nop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 195], OperandSize::Dword)
}

#[test]
fn nop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectDisplaced(EDX, 637107993, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 130, 25, 127, 249, 37], OperandSize::Dword)
}

#[test]
fn nop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 199], OperandSize::Qword)
}

#[test]
fn nop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 4, 199], OperandSize::Qword)
}

