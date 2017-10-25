use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 229, 82], OperandSize::Dword)
}

#[test]
fn pblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 44, 115, 98], OperandSize::Dword)
}

#[test]
fn pblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 200, 103], OperandSize::Qword)
}

#[test]
fn pblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 52, 209, 13], OperandSize::Qword)
}

