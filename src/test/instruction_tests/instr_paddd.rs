use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 222], OperandSize::Dword)
}

#[test]
fn paddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 184718592, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 172, 95, 0, 149, 2, 11], OperandSize::Dword)
}

#[test]
fn paddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 240], OperandSize::Qword)
}

#[test]
fn paddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 254, 20, 136], OperandSize::Qword)
}

#[test]
fn paddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 196], OperandSize::Dword)
}

#[test]
fn paddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 8], OperandSize::Dword)
}

#[test]
fn paddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 248], OperandSize::Qword)
}

#[test]
fn paddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 254, 36, 94], OperandSize::Qword)
}

