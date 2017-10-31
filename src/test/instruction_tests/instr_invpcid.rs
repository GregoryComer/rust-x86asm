use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invpcid_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1835307474, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 172, 138, 210, 145, 100, 109], OperandSize::Dword)
}

#[test]
fn invpcid_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1856298960, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 172, 88, 208, 223, 164, 110], OperandSize::Qword)
}

