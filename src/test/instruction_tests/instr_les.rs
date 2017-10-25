use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn les_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BP, 238, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 174, 238, 0], OperandSize::Word)
}

#[test]
fn les_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1126433243, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 196, 60, 149, 219, 1, 36, 67], OperandSize::Dword)
}

#[test]
fn les_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(ECX, 59137426, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 185, 146, 93, 134, 3], OperandSize::Dword)
}

