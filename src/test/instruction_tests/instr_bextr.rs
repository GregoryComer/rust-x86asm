use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bextr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 247, 237], OperandSize::Dword)
}

#[test]
fn bextr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EDI, 1574384675, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 247, 175, 35, 52, 215, 93], OperandSize::Dword)
}

#[test]
fn bextr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 247, 217], OperandSize::Qword)
}

#[test]
fn bextr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 627814865, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 247, 52, 133, 209, 177, 107, 37], OperandSize::Qword)
}

#[test]
fn bextr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 247, 234], OperandSize::Qword)
}

#[test]
fn bextr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 247, 60, 211], OperandSize::Qword)
}

