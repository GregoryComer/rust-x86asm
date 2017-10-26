use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 241], OperandSize::Dword)
}

#[test]
fn paddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 36, 88], OperandSize::Dword)
}

#[test]
fn paddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 219], OperandSize::Qword)
}

#[test]
fn paddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 20, 201], OperandSize::Qword)
}

#[test]
fn paddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 215], OperandSize::Dword)
}

#[test]
fn paddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 44, 70], OperandSize::Dword)
}

#[test]
fn paddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 230], OperandSize::Qword)
}

#[test]
fn paddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDI, 138415278, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 183, 174, 12, 64, 8], OperandSize::Qword)
}

