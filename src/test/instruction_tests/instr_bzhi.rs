use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bzhi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 245, 245], OperandSize::Dword)
}

#[test]
fn bzhi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 847596462, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 245, 12, 245, 174, 75, 133, 50], OperandSize::Dword)
}

#[test]
fn bzhi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 218], OperandSize::Qword)
}

#[test]
fn bzhi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 245, 18], OperandSize::Qword)
}

#[test]
fn bzhi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 245, 234], OperandSize::Qword)
}

#[test]
fn bzhi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RDI, 1847334136, Some(OperandSize::Qword), None)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 245, 151, 248, 20, 28, 110], OperandSize::Qword)
}

