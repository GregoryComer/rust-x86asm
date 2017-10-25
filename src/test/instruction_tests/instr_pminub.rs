use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 226], OperandSize::Dword)
}

#[test]
fn pminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 24], OperandSize::Dword)
}

#[test]
fn pminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 202], OperandSize::Qword)
}

#[test]
fn pminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1346942829, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 218, 132, 142, 109, 183, 72, 80], OperandSize::Qword)
}

#[test]
fn pminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 215], OperandSize::Dword)
}

#[test]
fn pminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 1], OperandSize::Dword)
}

#[test]
fn pminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 199], OperandSize::Qword)
}

#[test]
fn pminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 810088385, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 218, 172, 83, 193, 247, 72, 48], OperandSize::Qword)
}

