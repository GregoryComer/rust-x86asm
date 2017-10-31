use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bound_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(DI, 21254, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 149, 6, 83], OperandSize::Word)
}

#[test]
fn bound_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 44, 210], OperandSize::Dword)
}

#[test]
fn bound_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(SI, 251, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 140, 251, 0], OperandSize::Word)
}

#[test]
fn bound_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 52, 255], OperandSize::Dword)
}

