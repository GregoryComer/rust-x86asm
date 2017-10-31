use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movnti_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectDisplaced(SI, 6750, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 195, 164, 94, 26], OperandSize::Word)
}

#[test]
fn movnti_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 60, 120], OperandSize::Dword)
}

#[test]
fn movnti_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectDisplaced(RBX, 816798253, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 163, 45, 90, 175, 48], OperandSize::Qword)
}

#[test]
fn movnti_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 195, 35], OperandSize::Qword)
}

