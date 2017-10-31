use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 243], OperandSize::Dword)
}

#[test]
fn pcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1012134850, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 20, 213, 194, 243, 83, 60], OperandSize::Dword)
}

#[test]
fn pcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 215], OperandSize::Qword)
}

#[test]
fn pcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 2063939861, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 180, 91, 21, 57, 5, 123], OperandSize::Qword)
}

#[test]
fn pcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 255], OperandSize::Dword)
}

#[test]
fn pcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 2142275370, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 4, 221, 42, 135, 176, 127], OperandSize::Dword)
}

#[test]
fn pcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 247], OperandSize::Qword)
}

#[test]
fn pcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1130836595, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 131, 115, 50, 103, 67], OperandSize::Qword)
}

