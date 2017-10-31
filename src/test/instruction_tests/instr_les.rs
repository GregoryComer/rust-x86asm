use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn les_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(SI)), operand2: Some(Indirect(SI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 52], OperandSize::Word)
}

#[test]
fn les_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2107582669, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 196, 36, 197, 205, 40, 159, 125], OperandSize::Dword)
}

#[test]
fn les_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LES, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDI, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 55], OperandSize::Dword)
}

