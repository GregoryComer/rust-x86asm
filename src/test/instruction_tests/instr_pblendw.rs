use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 231, 74], OperandSize::Dword)
}

#[test]
fn pblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 8, 105], OperandSize::Dword)
}

#[test]
fn pblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 236, 60], OperandSize::Qword)
}

#[test]
fn pblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1959021625, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 180, 151, 57, 76, 196, 116, 58], OperandSize::Qword)
}

