use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 228, 67], OperandSize::Dword)
}

#[test]
fn pcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 411704735, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 20, 141, 159, 29, 138, 24, 10], OperandSize::Dword)
}

#[test]
fn pcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 221, 56], OperandSize::Qword)
}

#[test]
fn pcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 11, 26], OperandSize::Qword)
}

