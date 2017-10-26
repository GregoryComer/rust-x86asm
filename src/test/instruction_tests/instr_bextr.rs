use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bextr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 247, 228], OperandSize::Dword)
}

#[test]
fn bextr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 247, 36, 214], OperandSize::Dword)
}

#[test]
fn bextr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 247, 225], OperandSize::Qword)
}

#[test]
fn bextr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1449456340, Some(OperandSize::Dword), None)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 247, 20, 149, 212, 242, 100, 86], OperandSize::Qword)
}

#[test]
fn bextr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 247, 251], OperandSize::Qword)
}

#[test]
fn bextr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 247, 28, 67], OperandSize::Qword)
}

