use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Memory(30022, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 6, 70, 117], OperandSize::Word)
}

#[test]
fn fld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1697700142, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4, 245, 46, 217, 48, 101], OperandSize::Dword)
}

#[test]
fn fld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 1], OperandSize::Qword)
}

#[test]
fn fld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 198], OperandSize::Word)
}

#[test]
fn fld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 197], OperandSize::Dword)
}

#[test]
fn fld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 194], OperandSize::Qword)
}

#[test]
fn fld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(BX, 196, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 175, 196, 0], OperandSize::Word)
}

#[test]
fn fld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1611812741, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 172, 214, 133, 79, 18, 96], OperandSize::Dword)
}

#[test]
fn fld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 44, 65], OperandSize::Qword)
}

#[test]
fn fld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(BP, 14874, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 134, 26, 58], OperandSize::Word)
}

#[test]
fn fld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 4, 201], OperandSize::Dword)
}

#[test]
fn fld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 1], OperandSize::Qword)
}

