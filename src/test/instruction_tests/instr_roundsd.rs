use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 236, 85], OperandSize::Dword)
}

#[test]
fn roundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 17, 32], OperandSize::Dword)
}

#[test]
fn roundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 233, 36], OperandSize::Qword)
}

#[test]
fn roundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RCX, 1011988407, Some(OperandSize::Qword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 11, 145, 183, 183, 81, 60, 121], OperandSize::Qword)
}

