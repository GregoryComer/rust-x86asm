use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 243], OperandSize::Dword)
}

#[test]
fn pminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 154123635, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 132, 83, 115, 189, 47, 9], OperandSize::Dword)
}

#[test]
fn pminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 217], OperandSize::Qword)
}

#[test]
fn pminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RCX, 404132811, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 137, 203, 147, 22, 24], OperandSize::Qword)
}

#[test]
fn pminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 226], OperandSize::Dword)
}

#[test]
fn pminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1138784639, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 167, 127, 121, 224, 67], OperandSize::Dword)
}

#[test]
fn pminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 216], OperandSize::Qword)
}

#[test]
fn pminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 60, 94], OperandSize::Qword)
}

