use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 254], OperandSize::Dword)
}

#[test]
fn addsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1077323015, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 188, 219, 7, 165, 54, 64], OperandSize::Dword)
}

#[test]
fn addsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 220], OperandSize::Qword)
}

#[test]
fn addsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1591982333, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 176, 253, 184, 227, 94], OperandSize::Qword)
}

