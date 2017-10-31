use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fimul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(BX, 20417, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 143, 193, 79], OperandSize::Word)
}

#[test]
fn fimul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 12, 155], OperandSize::Dword)
}

#[test]
fn fimul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(RAX, 617510786, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 136, 130, 119, 206, 36], OperandSize::Qword)
}

#[test]
fn fimul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10273, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 136, 33, 40], OperandSize::Word)
}

#[test]
fn fimul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(ESI, 1098385454, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 142, 46, 8, 120, 65], OperandSize::Dword)
}

#[test]
fn fimul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 208], OperandSize::Qword)
}

