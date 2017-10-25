use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rorx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 250, 4], OperandSize::Dword)
}

#[test]
fn rorx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 519098371, Some(OperandSize::Dword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 28, 245, 3, 208, 240, 30, 43], OperandSize::Dword)
}

#[test]
fn rorx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 233, 123], OperandSize::Qword)
}

#[test]
fn rorx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 60, 122, 15], OperandSize::Qword)
}

#[test]
fn rorx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 245, 40], OperandSize::Qword)
}

#[test]
fn rorx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RSI, 31017343, Some(OperandSize::Qword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 150, 127, 73, 217, 1, 48], OperandSize::Qword)
}

