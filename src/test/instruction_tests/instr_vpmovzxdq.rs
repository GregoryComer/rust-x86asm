use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 196], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 651756842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 129, 42, 5, 217, 38], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 238], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 36, 191], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 212], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2139468609, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 44, 189, 65, 179, 133, 127], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 192], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 49], OperandSize::Qword)
}

