use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bound_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(SI, 106, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 92, 106], OperandSize::Word)
}

#[test]
fn bound_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 107104591, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 12, 85, 79, 73, 98, 6], OperandSize::Dword)
}

#[test]
fn bound_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 216, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 191, 216, 0], OperandSize::Word)
}

#[test]
fn bound_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 44, 83], OperandSize::Dword)
}

