use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 223], OperandSize::Dword)
}

#[test]
fn mulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 51], OperandSize::Dword)
}

#[test]
fn mulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 217], OperandSize::Qword)
}

#[test]
fn mulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 674850177, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 137, 129, 101, 57, 40], OperandSize::Qword)
}

