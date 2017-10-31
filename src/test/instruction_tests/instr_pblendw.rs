use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 246, 107], OperandSize::Dword)
}

#[test]
fn pblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 252517184, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 161, 64, 27, 13, 15, 103], OperandSize::Dword)
}

#[test]
fn pblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 214, 123], OperandSize::Qword)
}

#[test]
fn pblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 1199056203, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 138, 75, 37, 120, 71, 125], OperandSize::Qword)
}

