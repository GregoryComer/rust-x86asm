use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movnti_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 151, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 195, 169, 151, 0], OperandSize::Word)
}

#[test]
fn movnti_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1591196460, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 140, 194, 44, 187, 215, 94], OperandSize::Dword)
}

#[test]
fn movnti_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectDisplaced(RBX, 2131616449, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 163, 193, 226, 13, 127], OperandSize::Qword)
}

#[test]
fn movnti_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledDisplaced(RDI, Four, 504401495, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 195, 60, 189, 87, 142, 16, 30], OperandSize::Qword)
}

