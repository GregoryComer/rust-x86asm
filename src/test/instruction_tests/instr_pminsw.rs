use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 255], OperandSize::Dword)
}

#[test]
fn pminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1951202420, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 188, 135, 116, 252, 76, 116], OperandSize::Dword)
}

#[test]
fn pminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 241], OperandSize::Qword)
}

#[test]
fn pminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 46544918, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 140, 152, 22, 56, 198, 2], OperandSize::Qword)
}

#[test]
fn pminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 255], OperandSize::Dword)
}

#[test]
fn pminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 828331904, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 60, 85, 128, 87, 95, 49], OperandSize::Dword)
}

#[test]
fn pminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 206], OperandSize::Qword)
}

#[test]
fn pminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1048617806, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 131, 78, 163, 128, 62], OperandSize::Qword)
}

