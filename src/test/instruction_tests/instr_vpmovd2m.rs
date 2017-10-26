use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovd2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 57, 208], OperandSize::Dword)
}

#[test]
fn vpmovd2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 8, 57, 232], OperandSize::Qword)
}

#[test]
fn vpmovd2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 57, 247], OperandSize::Dword)
}

#[test]
fn vpmovd2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 40, 57, 240], OperandSize::Qword)
}

#[test]
fn vpmovd2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 57, 255], OperandSize::Dword)
}

#[test]
fn vpmovd2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVD2M, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 57, 249], OperandSize::Qword)
}

