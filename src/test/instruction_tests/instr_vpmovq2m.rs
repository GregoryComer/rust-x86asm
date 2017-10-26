use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovq2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 57, 235], OperandSize::Dword)
}

#[test]
fn vpmovq2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 254, 8, 57, 252], OperandSize::Qword)
}

#[test]
fn vpmovq2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 57, 231], OperandSize::Dword)
}

#[test]
fn vpmovq2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 146, 254, 40, 57, 213], OperandSize::Qword)
}

#[test]
fn vpmovq2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 57, 229], OperandSize::Dword)
}

#[test]
fn vpmovq2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQ2M, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 254, 72, 57, 216], OperandSize::Qword)
}

