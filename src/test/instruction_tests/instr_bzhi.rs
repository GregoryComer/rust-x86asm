use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bzhi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 245, 203], OperandSize::Dword)
}

#[test]
fn bzhi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 245, 60, 198], OperandSize::Dword)
}

#[test]
fn bzhi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 245, 245], OperandSize::Qword)
}

#[test]
fn bzhi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RAX, 1875149779, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 136, 211, 131, 196, 111], OperandSize::Qword)
}

#[test]
fn bzhi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 245, 210], OperandSize::Qword)
}

#[test]
fn bzhi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 245, 36, 154], OperandSize::Qword)
}

