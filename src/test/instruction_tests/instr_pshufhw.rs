use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 219, 112], OperandSize::Dword)
}

#[test]
fn pshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1092577326, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 148, 203, 46, 104, 31, 65, 70], OperandSize::Dword)
}

#[test]
fn pshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 217, 74], OperandSize::Qword)
}

#[test]
fn pshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 2034078860, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 129, 140, 148, 61, 121, 66], OperandSize::Qword)
}

