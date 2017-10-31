use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 224, 33], OperandSize::Dword)
}

#[test]
fn cmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 12, 158, 110], OperandSize::Dword)
}

#[test]
fn cmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 202, 50], OperandSize::Qword)
}

#[test]
fn cmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 44, 89, 87], OperandSize::Qword)
}

