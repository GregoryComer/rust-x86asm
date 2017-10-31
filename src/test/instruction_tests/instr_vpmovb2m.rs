use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovb2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 249], OperandSize::Dword)
}

#[test]
fn vpmovb2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 8, 41, 250], OperandSize::Qword)
}

#[test]
fn vpmovb2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 41, 237], OperandSize::Dword)
}

#[test]
fn vpmovb2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 40, 41, 200], OperandSize::Qword)
}

#[test]
fn vpmovb2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 237], OperandSize::Dword)
}

#[test]
fn vpmovb2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 126, 72, 41, 235], OperandSize::Qword)
}

