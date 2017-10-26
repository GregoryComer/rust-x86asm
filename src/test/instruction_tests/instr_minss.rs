use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 238], OperandSize::Dword)
}

#[test]
fn minss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 2138864498, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 164, 87, 114, 123, 124, 127], OperandSize::Dword)
}

#[test]
fn minss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 207], OperandSize::Qword)
}

#[test]
fn minss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 28, 81], OperandSize::Qword)
}

