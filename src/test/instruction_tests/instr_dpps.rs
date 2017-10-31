use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 244, 114], OperandSize::Dword)
}

#[test]
fn dpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 259952946, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 12, 125, 50, 145, 126, 15, 122], OperandSize::Dword)
}

#[test]
fn dpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 228, 83], OperandSize::Qword)
}

#[test]
fn dpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 44, 72, 40], OperandSize::Qword)
}

