use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 221], OperandSize::Dword)
}

#[test]
fn movups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 1], OperandSize::Dword)
}

#[test]
fn movups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 223], OperandSize::Qword)
}

#[test]
fn movups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 1395495468, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 130, 44, 146, 45, 83], OperandSize::Qword)
}

#[test]
fn movups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 230], OperandSize::Dword)
}

#[test]
fn movups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 912832369, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 20, 253, 113, 183, 104, 54], OperandSize::Dword)
}

#[test]
fn movups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 202], OperandSize::Qword)
}

#[test]
fn movups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectDisplaced(RDI, 1719948781, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 167, 237, 85, 132, 102], OperandSize::Qword)
}

