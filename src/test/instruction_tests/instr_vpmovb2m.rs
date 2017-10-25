use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovb2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 248], OperandSize::Dword)
}

#[test]
fn vpmovb2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 224], OperandSize::Qword)
}

#[test]
fn vpmovb2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 41, 235], OperandSize::Dword)
}

#[test]
fn vpmovb2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 40, 41, 245], OperandSize::Qword)
}

#[test]
fn vpmovb2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 206], OperandSize::Dword)
}

#[test]
fn vpmovb2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 225], OperandSize::Qword)
}

