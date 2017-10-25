use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 234], OperandSize::Dword)
}

#[test]
fn mulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 60, 90], OperandSize::Dword)
}

#[test]
fn mulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 235], OperandSize::Qword)
}

#[test]
fn mulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 368962757, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 164, 208, 197, 236, 253, 21], OperandSize::Qword)
}

