use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rorx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 228, 27], OperandSize::Dword)
}

#[test]
fn rorx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 52, 216, 40], OperandSize::Dword)
}

#[test]
fn rorx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 226, 19], OperandSize::Qword)
}

#[test]
fn rorx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 60, 214, 106], OperandSize::Qword)
}

#[test]
fn rorx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 238, 3], OperandSize::Qword)
}

#[test]
fn rorx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1278822464, Some(OperandSize::Qword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 140, 208, 64, 72, 57, 76, 12], OperandSize::Qword)
}

