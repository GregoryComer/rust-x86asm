use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 193], OperandSize::Dword)
}

#[test]
fn andps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 4, 95], OperandSize::Dword)
}

#[test]
fn andps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 232], OperandSize::Qword)
}

#[test]
fn andps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1190408904, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 28, 69, 200, 50, 244, 70], OperandSize::Qword)
}

