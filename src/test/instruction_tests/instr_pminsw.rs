use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 201], OperandSize::Dword)
}

#[test]
fn pminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 12, 248], OperandSize::Dword)
}

#[test]
fn pminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 215], OperandSize::Qword)
}

#[test]
fn pminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 2003630321, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 132, 194, 241, 248, 108, 119], OperandSize::Qword)
}

#[test]
fn pminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 226], OperandSize::Dword)
}

#[test]
fn pminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 343742146, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 188, 151, 194, 22, 125, 20], OperandSize::Dword)
}

#[test]
fn pminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 195], OperandSize::Qword)
}

#[test]
fn pminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 567966900, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 172, 194, 180, 124, 218, 33], OperandSize::Qword)
}

