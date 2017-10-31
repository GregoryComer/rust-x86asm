use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 223, 80], OperandSize::Dword)
}

#[test]
fn dppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1454145160, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 52, 117, 136, 126, 172, 86, 93], OperandSize::Dword)
}

#[test]
fn dppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 247, 53], OperandSize::Qword)
}

#[test]
fn dppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RSI, 2070692297, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 166, 201, 65, 108, 123, 114], OperandSize::Qword)
}

