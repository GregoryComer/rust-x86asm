use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lds_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25967, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 163, 111, 101], OperandSize::Word)
}

#[test]
fn lds_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 933714845, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 197, 44, 245, 157, 91, 167, 55], OperandSize::Dword)
}

#[test]
fn lds_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1811434795, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 148, 123, 43, 77, 248, 107], OperandSize::Dword)
}

