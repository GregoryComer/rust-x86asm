use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bound_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 137, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 171, 137, 0], OperandSize::Word)
}

#[test]
fn bound_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 20, 66], OperandSize::Dword)
}

#[test]
fn bound_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 75, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 75, 75], OperandSize::Word)
}

#[test]
fn bound_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDX, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 50], OperandSize::Dword)
}

