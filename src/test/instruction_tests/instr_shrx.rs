use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 247, 201], OperandSize::Dword)
}

#[test]
fn shrx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 799838310, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 247, 12, 117, 102, 144, 172, 47], OperandSize::Dword)
}

#[test]
fn shrx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 247, 210], OperandSize::Qword)
}

#[test]
fn shrx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RAX, 1839369608, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 247, 144, 136, 141, 162, 109], OperandSize::Qword)
}

#[test]
fn shrx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 219, 247, 203], OperandSize::Qword)
}

#[test]
fn shrx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RDX, 556903017, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 235, 247, 170, 105, 170, 49, 33], OperandSize::Qword)
}

