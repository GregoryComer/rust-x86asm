use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shlx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 247, 239], OperandSize::Dword)
}

#[test]
fn shlx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EDI, 426249900, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 247, 151, 172, 14, 104, 25], OperandSize::Dword)
}

#[test]
fn shlx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 247, 207], OperandSize::Qword)
}

#[test]
fn shlx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 247, 34], OperandSize::Qword)
}

#[test]
fn shlx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 247, 215], OperandSize::Qword)
}

#[test]
fn shlx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RDX, 1901033199, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 247, 186, 239, 118, 79, 113], OperandSize::Qword)
}

