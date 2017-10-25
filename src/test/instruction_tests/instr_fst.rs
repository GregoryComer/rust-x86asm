use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fst_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 14205, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 145, 125, 55], OperandSize::Word)
}

#[test]
fn fst_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(EDI, 1829514694, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 151, 198, 45, 12, 109], OperandSize::Dword)
}

#[test]
fn fst_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1484144133, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 149, 5, 62, 118, 88], OperandSize::Qword)
}

#[test]
fn fst_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(SI, 118, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 84, 118], OperandSize::Word)
}

#[test]
fn fst_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 20, 143], OperandSize::Dword)
}

#[test]
fn fst_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(RCX, 1851467886, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 145, 110, 40, 91, 110], OperandSize::Qword)
}

#[test]
fn fst_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 210], OperandSize::Word)
}

#[test]
fn fst_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 210], OperandSize::Dword)
}

#[test]
fn fst_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 215], OperandSize::Qword)
}

