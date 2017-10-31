use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lds_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 8, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 8], OperandSize::Word)
}

#[test]
fn lds_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(ECX, 923611318, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 197, 161, 182, 48, 13, 55], OperandSize::Dword)
}

#[test]
fn lds_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EBX, 36909671, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 155, 103, 50, 51, 2], OperandSize::Dword)
}

