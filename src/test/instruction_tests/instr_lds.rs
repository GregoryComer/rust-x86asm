use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lds_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 0, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 57], OperandSize::Word)
}

#[test]
fn lds_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1688992813, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 197, 20, 221, 45, 252, 171, 100], OperandSize::Dword)
}

#[test]
fn lds_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDI, 135138030, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 143, 238, 10, 14, 8], OperandSize::Dword)
}

