use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 252], OperandSize::Dword)
}

#[test]
fn aesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 30], OperandSize::Dword)
}

#[test]
fn aesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 241], OperandSize::Qword)
}

#[test]
fn aesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENC, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 162002391, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 220, 4, 125, 215, 245, 167, 9], OperandSize::Qword)
}

