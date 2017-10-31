use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovd2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 57, 237], OperandSize::Dword)
}

#[test]
fn vpmovd2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 8, 57, 231], OperandSize::Qword)
}

#[test]
fn vpmovd2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 57, 203], OperandSize::Dword)
}

#[test]
fn vpmovd2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 40, 57, 230], OperandSize::Qword)
}

#[test]
fn vpmovd2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 57, 211], OperandSize::Dword)
}

#[test]
fn vpmovd2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 72, 57, 242], OperandSize::Qword)
}

