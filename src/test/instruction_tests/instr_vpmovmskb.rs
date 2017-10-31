use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 207], OperandSize::Dword)
}

#[test]
fn vpmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 215, 249], OperandSize::Qword)
}

#[test]
fn vpmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(ESI)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 242], OperandSize::Dword)
}

#[test]
fn vpmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVMSKB, operand1: Some(Direct(RBX)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 215, 218], OperandSize::Qword)
}

