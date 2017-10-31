use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 243], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 44, 64], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 220], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 43], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 237], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 381529309, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 44, 181, 221, 172, 189, 22], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 225], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 20, 65], OperandSize::Qword)
}

