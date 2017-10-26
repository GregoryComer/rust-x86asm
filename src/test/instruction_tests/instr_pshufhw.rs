use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 203, 57], OperandSize::Dword)
}

#[test]
fn pshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1898135758, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 44, 189, 206, 64, 35, 113, 109], OperandSize::Dword)
}

#[test]
fn pshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 255, 12], OperandSize::Qword)
}

#[test]
fn pshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 48, 109], OperandSize::Qword)
}

