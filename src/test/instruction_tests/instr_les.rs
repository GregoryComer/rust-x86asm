use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn les_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 29756, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 190, 60, 116], OperandSize::Word)
}

#[test]
fn les_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1295594029, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 196, 164, 129, 45, 50, 57, 77], OperandSize::Dword)
}

#[test]
fn les_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 77212781, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 52, 69, 109, 44, 154, 4], OperandSize::Dword)
}

