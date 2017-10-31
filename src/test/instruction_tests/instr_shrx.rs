use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 247, 222], OperandSize::Dword)
}

#[test]
fn shrx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 247, 48], OperandSize::Dword)
}

#[test]
fn shrx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 247, 227], OperandSize::Qword)
}

#[test]
fn shrx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 247, 52, 211], OperandSize::Qword)
}

#[test]
fn shrx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 243, 247, 221], OperandSize::Qword)
}

#[test]
fn shrx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 219, 247, 12, 178], OperandSize::Qword)
}

