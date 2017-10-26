use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovb2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 220], OperandSize::Dword)
}

#[test]
fn vpmovb2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 126, 8, 41, 239], OperandSize::Qword)
}

#[test]
fn vpmovb2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 41, 232], OperandSize::Dword)
}

#[test]
fn vpmovb2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 40, 41, 233], OperandSize::Qword)
}

#[test]
fn vpmovb2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 255], OperandSize::Dword)
}

#[test]
fn vpmovb2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 72, 41, 213], OperandSize::Qword)
}

