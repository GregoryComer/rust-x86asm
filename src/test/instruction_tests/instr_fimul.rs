use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fimul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(BP, 47, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 78, 47], OperandSize::Word)
}

#[test]
fn fimul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1151590181, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 140, 73, 37, 223, 163, 68], OperandSize::Dword)
}

#[test]
fn fimul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 12, 147], OperandSize::Qword)
}

#[test]
fn fimul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 14223, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 136, 143, 55], OperandSize::Word)
}

#[test]
fn fimul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 190], OperandSize::Dword)
}

#[test]
fn fimul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(RBX, 1859206109, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 139, 221, 59, 209, 110], OperandSize::Qword)
}

