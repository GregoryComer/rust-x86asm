use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pext_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 74, 245, 220], OperandSize::Dword)
}

#[test]
fn pext_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1010806382, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 245, 28, 253, 110, 174, 63, 60], OperandSize::Dword)
}

#[test]
fn pext_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 245, 210], OperandSize::Qword)
}

#[test]
fn pext_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: Some(IndirectDisplaced(RBX, 2020384250, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 66, 245, 163, 250, 157, 108, 120], OperandSize::Qword)
}

#[test]
fn pext_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 210, 245, 252], OperandSize::Qword)
}

#[test]
fn pext_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDI)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 2062985082, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 194, 245, 164, 219, 122, 167, 246, 122], OperandSize::Qword)
}

