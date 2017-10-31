use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shlx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 247, 221], OperandSize::Dword)
}

#[test]
fn shlx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EDI, 1954795553, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 247, 167, 33, 208, 131, 116], OperandSize::Dword)
}

#[test]
fn shlx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 247, 245], OperandSize::Qword)
}

#[test]
fn shlx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1004906067, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 247, 164, 248, 83, 166, 229, 59], OperandSize::Qword)
}

#[test]
fn shlx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 247, 223], OperandSize::Qword)
}

#[test]
fn shlx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 851895828, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 247, 138, 20, 230, 198, 50], OperandSize::Qword)
}

