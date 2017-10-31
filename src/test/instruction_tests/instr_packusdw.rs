use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 208], OperandSize::Dword)
}

#[test]
fn packusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1054303909, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 52, 93, 165, 102, 215, 62], OperandSize::Dword)
}

#[test]
fn packusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 228], OperandSize::Qword)
}

#[test]
fn packusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 52, 194], OperandSize::Qword)
}

