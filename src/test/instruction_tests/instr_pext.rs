use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pext_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 255], OperandSize::Dword)
}

#[test]
fn pext_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 691164690, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 66, 245, 12, 245, 18, 86, 50, 41], OperandSize::Dword)
}

#[test]
fn pext_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 74, 245, 220], OperandSize::Qword)
}

#[test]
fn pext_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 245, 58], OperandSize::Qword)
}

#[test]
fn pext_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 245, 245], OperandSize::Qword)
}

#[test]
fn pext_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: Some(IndirectDisplaced(RSI, 426440379, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 210, 245, 150, 187, 246, 106, 25], OperandSize::Qword)
}

