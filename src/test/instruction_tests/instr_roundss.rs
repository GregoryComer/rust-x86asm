use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 250, 109], OperandSize::Dword)
}

#[test]
fn roundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 43, 64], OperandSize::Dword)
}

#[test]
fn roundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 193, 55], OperandSize::Qword)
}

#[test]
fn roundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 235573831, Some(OperandSize::Dword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 156, 147, 71, 146, 10, 14, 8], OperandSize::Qword)
}

