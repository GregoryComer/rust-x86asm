use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bound_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 25], OperandSize::Word)
}

#[test]
fn bound_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 44, 64], OperandSize::Dword)
}

#[test]
fn bound_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(EDX)), operand2: Some(Indirect(BX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 23], OperandSize::Word)
}

#[test]
fn bound_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(ECX)), operand2: Some(Indirect(ESI, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 14], OperandSize::Dword)
}

