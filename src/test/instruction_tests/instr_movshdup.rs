use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 237], OperandSize::Dword)
}

#[test]
fn movshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 4, 139], OperandSize::Dword)
}

#[test]
fn movshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 201], OperandSize::Qword)
}

#[test]
fn movshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSHDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1209082648, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 22, 140, 199, 24, 35, 17, 72], OperandSize::Qword)
}

