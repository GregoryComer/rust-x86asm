use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 242], OperandSize::Dword)
}

#[test]
fn pminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 36, 250], OperandSize::Dword)
}

#[test]
fn pminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 206], OperandSize::Qword)
}

#[test]
fn pminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RBX, 395519386, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 171, 154, 37, 147, 23], OperandSize::Qword)
}

#[test]
fn pminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 242], OperandSize::Dword)
}

#[test]
fn pminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 16], OperandSize::Dword)
}

#[test]
fn pminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 228], OperandSize::Qword)
}

#[test]
fn pminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 52, 202], OperandSize::Qword)
}

