use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 214, 96], OperandSize::Dword)
}

#[test]
fn dppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 278697177, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 156, 67, 217, 148, 156, 16, 3], OperandSize::Dword)
}

#[test]
fn dppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 208, 121], OperandSize::Qword)
}

#[test]
fn dppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2084693867, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 36, 157, 107, 231, 65, 124, 92], OperandSize::Qword)
}

