use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovb2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 227], OperandSize::Dword)
}

#[test]
fn vpmovb2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 41, 202], OperandSize::Qword)
}

#[test]
fn vpmovb2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 41, 247], OperandSize::Dword)
}

#[test]
fn vpmovb2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 40, 41, 218], OperandSize::Qword)
}

#[test]
fn vpmovb2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 41, 246], OperandSize::Dword)
}

#[test]
fn vpmovb2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVB2M, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 126, 72, 41, 252], OperandSize::Qword)
}

