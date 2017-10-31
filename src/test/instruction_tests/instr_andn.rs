use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 242, 235], OperandSize::Dword)
}

#[test]
fn andn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 48], OperandSize::Dword)
}

#[test]
fn andn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 242, 234], OperandSize::Qword)
}

#[test]
fn andn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 242, 12, 114], OperandSize::Qword)
}

#[test]
fn andn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 242, 246], OperandSize::Qword)
}

#[test]
fn andn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1515331957, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 242, 164, 130, 117, 33, 82, 90], OperandSize::Qword)
}

