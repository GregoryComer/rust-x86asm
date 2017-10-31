use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shlx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 247, 213], OperandSize::Dword)
}

#[test]
fn shlx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1646487721, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 247, 172, 190, 169, 104, 35, 98], OperandSize::Dword)
}

#[test]
fn shlx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 247, 201], OperandSize::Qword)
}

#[test]
fn shlx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 941868864, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 247, 188, 86, 64, 199, 35, 56], OperandSize::Qword)
}

#[test]
fn shlx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 247, 212], OperandSize::Qword)
}

#[test]
fn shlx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 247, 55], OperandSize::Qword)
}

