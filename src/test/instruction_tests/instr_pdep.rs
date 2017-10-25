use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pdep_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 245, 243], OperandSize::Dword)
}

#[test]
fn pdep_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 245, 14], OperandSize::Dword)
}

#[test]
fn pdep_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 245, 202], OperandSize::Qword)
}

#[test]
fn pdep_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: Some(IndirectDisplaced(RDI, 1171136822, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 245, 151, 54, 33, 206, 69], OperandSize::Qword)
}

#[test]
fn pdep_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 245, 213], OperandSize::Qword)
}

#[test]
fn pdep_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 369950915, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 195, 245, 172, 139, 195, 0, 13, 22], OperandSize::Qword)
}

