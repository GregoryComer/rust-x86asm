use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovw2m_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 41, 236], OperandSize::Dword)
}

#[test]
fn vpmovw2m_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 178, 254, 8, 41, 249], OperandSize::Qword)
}

#[test]
fn vpmovw2m_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 41, 210], OperandSize::Dword)
}

#[test]
fn vpmovw2m_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 254, 40, 41, 209], OperandSize::Qword)
}

#[test]
fn vpmovw2m_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 41, 230], OperandSize::Dword)
}

#[test]
fn vpmovw2m_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVW2M, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 210, 254, 72, 41, 218], OperandSize::Qword)
}

