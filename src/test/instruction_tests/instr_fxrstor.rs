use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 11], OperandSize::Word)
}

#[test]
fn fxrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 12, 89], OperandSize::Dword)
}

#[test]
fn fxrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 12, 130], OperandSize::Qword)
}

