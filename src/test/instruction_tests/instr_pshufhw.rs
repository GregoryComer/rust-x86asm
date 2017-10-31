use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 245, 112], OperandSize::Dword)
}

#[test]
fn pshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 668490069, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 140, 137, 85, 89, 216, 39, 99], OperandSize::Dword)
}

#[test]
fn pshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 195, 126], OperandSize::Qword)
}

#[test]
fn pshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 44, 210, 64], OperandSize::Qword)
}

