use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 215], OperandSize::Dword)
}

#[test]
fn movsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 237], OperandSize::Qword)
}

#[test]
fn movsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 721171315, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 172, 121, 115, 51, 252, 42], OperandSize::Dword)
}

#[test]
fn movsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 48], OperandSize::Qword)
}

#[test]
fn movsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 227], OperandSize::Dword)
}

#[test]
fn movsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 46], OperandSize::Dword)
}

#[test]
fn movsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 207], OperandSize::Qword)
}

#[test]
fn movsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 44, 75], OperandSize::Qword)
}

