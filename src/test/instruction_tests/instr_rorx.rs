use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rorx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 206, 58], OperandSize::Dword)
}

#[test]
fn rorx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ECX)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 11, 100], OperandSize::Dword)
}

#[test]
fn rorx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 206, 69], OperandSize::Qword)
}

#[test]
fn rorx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1563838020, Some(OperandSize::Dword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 156, 142, 68, 70, 54, 93, 71], OperandSize::Qword)
}

#[test]
fn rorx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 204, 88], OperandSize::Qword)
}

#[test]
fn rorx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 16, 15], OperandSize::Qword)
}

