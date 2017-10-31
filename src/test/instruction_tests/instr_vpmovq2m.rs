use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovq2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 226], OperandSize::Dword)
}

#[test]
fn vpmovq2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 235], OperandSize::Qword)
}

#[test]
fn vpmovq2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 57, 232], OperandSize::Dword)
}

#[test]
fn vpmovq2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 57, 250], OperandSize::Qword)
}

#[test]
fn vpmovq2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 57, 219], OperandSize::Dword)
}

#[test]
fn vpmovq2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 254, 72, 57, 213], OperandSize::Qword)
}

