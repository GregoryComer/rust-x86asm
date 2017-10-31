use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 223], OperandSize::Dword)
}

#[test]
fn punpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 994891363, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 164, 217, 99, 214, 76, 59], OperandSize::Dword)
}

#[test]
fn punpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 212], OperandSize::Qword)
}

#[test]
fn punpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 2134900991, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 4, 189, 255, 0, 64, 127], OperandSize::Qword)
}

#[test]
fn punpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 230], OperandSize::Dword)
}

#[test]
fn punpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 32432456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 4, 149, 72, 225, 238, 1], OperandSize::Dword)
}

#[test]
fn punpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 248], OperandSize::Qword)
}

#[test]
fn punpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 2183631, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 132, 216, 207, 81, 33, 0], OperandSize::Qword)
}

