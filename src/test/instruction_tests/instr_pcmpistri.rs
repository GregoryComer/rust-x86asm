use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 253, 65], OperandSize::Dword)
}

#[test]
fn pcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 826454620, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 151, 92, 178, 66, 49, 28], OperandSize::Dword)
}

#[test]
fn pcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 196, 50], OperandSize::Qword)
}

#[test]
fn pcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1850890174, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 52, 245, 190, 87, 82, 110, 84], OperandSize::Qword)
}

