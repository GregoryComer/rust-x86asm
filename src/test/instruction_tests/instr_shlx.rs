use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shlx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 247, 244], OperandSize::Dword)
}

#[test]
fn shlx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 247, 52, 214], OperandSize::Dword)
}

#[test]
fn shlx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 247, 207], OperandSize::Qword)
}

#[test]
fn shlx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1233687723, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 247, 172, 73, 171, 148, 136, 73], OperandSize::Qword)
}

#[test]
fn shlx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 247, 231], OperandSize::Qword)
}

#[test]
fn shlx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 663695532, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 247, 180, 250, 172, 48, 143, 39], OperandSize::Qword)
}

