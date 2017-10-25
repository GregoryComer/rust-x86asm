use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovq2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 242], OperandSize::Dword)
}

#[test]
fn vpmovq2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 247], OperandSize::Qword)
}

#[test]
fn vpmovq2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 57, 221], OperandSize::Dword)
}

#[test]
fn vpmovq2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 254, 40, 57, 233], OperandSize::Qword)
}

#[test]
fn vpmovq2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 57, 236], OperandSize::Dword)
}

#[test]
fn vpmovq2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 254, 72, 57, 214], OperandSize::Qword)
}

