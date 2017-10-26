use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pext_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 245, 245], OperandSize::Dword)
}

#[test]
fn pext_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 74, 245, 28, 215], OperandSize::Dword)
}

#[test]
fn pext_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 245, 229], OperandSize::Qword)
}

#[test]
fn pext_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1728103558, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 106, 245, 140, 136, 134, 196, 0, 103], OperandSize::Qword)
}

#[test]
fn pext_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 218, 245, 250], OperandSize::Qword)
}

#[test]
fn pext_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 245, 20, 184], OperandSize::Qword)
}

