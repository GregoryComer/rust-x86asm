use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 249], OperandSize::Dword)
}

#[test]
fn divsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 640523327, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 180, 70, 63, 156, 45, 38], OperandSize::Dword)
}

#[test]
fn divsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 232], OperandSize::Qword)
}

#[test]
fn divsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 28, 215], OperandSize::Qword)
}

