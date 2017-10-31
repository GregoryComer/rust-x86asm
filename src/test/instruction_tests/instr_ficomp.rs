use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectDisplaced(BX, 231, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 159, 231, 0], OperandSize::Word)
}

#[test]
fn ficomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 27], OperandSize::Dword)
}

#[test]
fn ficomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 190], OperandSize::Qword)
}

#[test]
fn ficomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 24], OperandSize::Word)
}

#[test]
fn ficomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 28, 243], OperandSize::Dword)
}

#[test]
fn ficomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 26], OperandSize::Qword)
}

