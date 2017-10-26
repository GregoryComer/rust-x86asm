use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 222], OperandSize::Dword)
}

#[test]
fn pmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 1764300529, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 187, 241, 22, 41, 105], OperandSize::Dword)
}

#[test]
fn pmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 229], OperandSize::Qword)
}

#[test]
fn pmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1896240920, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 164, 142, 24, 87, 6, 113], OperandSize::Qword)
}

