use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 219], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1550586375, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 188, 154, 7, 18, 108, 92], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 236], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 12, 142], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 216], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EAX, 1812242852, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 176, 164, 161, 4, 108], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 195], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1573888161, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 20, 93, 161, 160, 207, 93], OperandSize::Qword)
}

