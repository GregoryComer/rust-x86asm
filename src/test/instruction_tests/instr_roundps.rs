use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 216, 66], OperandSize::Dword)
}

#[test]
fn roundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 12, 152, 66], OperandSize::Dword)
}

#[test]
fn roundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 248, 126], OperandSize::Qword)
}

#[test]
fn roundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 194499569, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 140, 203, 241, 211, 151, 11, 22], OperandSize::Qword)
}

