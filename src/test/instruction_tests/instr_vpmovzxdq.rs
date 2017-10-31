use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 238], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1069005223, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 12, 125, 167, 185, 183, 63], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 225], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1415285543, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 131, 39, 139, 91, 84], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 218], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 367292866, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 20, 125, 194, 113, 228, 21], OperandSize::Dword)
}

#[test]
fn vpmovzxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 244], OperandSize::Qword)
}

#[test]
fn vpmovzxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RDI, 1353739904, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 167, 128, 110, 176, 80], OperandSize::Qword)
}

