use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4], OperandSize::Word)
}

#[test]
fn fld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4, 70], OperandSize::Dword)
}

#[test]
fn fld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1519748210, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 132, 198, 114, 132, 149, 90], OperandSize::Qword)
}

#[test]
fn fld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 198], OperandSize::Word)
}

#[test]
fn fld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 193], OperandSize::Dword)
}

#[test]
fn fld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 199], OperandSize::Qword)
}

#[test]
fn fld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(BP, 14475, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 174, 139, 56], OperandSize::Word)
}

#[test]
fn fld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(EAX, 792621519, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 168, 207, 113, 62, 47], OperandSize::Dword)
}

#[test]
fn fld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 44, 223], OperandSize::Qword)
}

#[test]
fn fld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(BX, 38, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 71, 38], OperandSize::Word)
}

#[test]
fn fld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(EDI, 22622173, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 135, 221, 47, 89, 1], OperandSize::Dword)
}

#[test]
fn fld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 345266890, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 132, 195, 202, 90, 148, 20], OperandSize::Qword)
}

