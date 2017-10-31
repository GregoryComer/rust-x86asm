use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 3], OperandSize::Word)
}

#[test]
fn fadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 6], OperandSize::Dword)
}

#[test]
fn fadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 3], OperandSize::Qword)
}

#[test]
fn fadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 198], OperandSize::Word)
}

#[test]
fn fadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 198], OperandSize::Dword)
}

#[test]
fn fadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 196], OperandSize::Qword)
}

#[test]
fn fadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 192, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 129, 192, 0], OperandSize::Word)
}

#[test]
fn fadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 1], OperandSize::Dword)
}

#[test]
fn fadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1005110687, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 132, 159, 159, 197, 232, 59], OperandSize::Qword)
}

#[test]
fn fadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 198], OperandSize::Word)
}

#[test]
fn fadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 198], OperandSize::Dword)
}

#[test]
fn fadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 195], OperandSize::Qword)
}

