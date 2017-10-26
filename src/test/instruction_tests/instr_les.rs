use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn les_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 6175, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 162, 31, 24], OperandSize::Word)
}

#[test]
fn les_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(ECX, 898011182, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 196, 161, 46, 144, 134, 53], OperandSize::Dword)
}

#[test]
fn les_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ESI, 974571035, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 158, 27, 198, 22, 58], OperandSize::Dword)
}

