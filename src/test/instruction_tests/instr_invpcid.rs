use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn invpcid_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 20, 200], OperandSize::Dword)
}

#[test]
fn invpcid_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVPCID, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RCX, 1708849368, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 130, 161, 216, 248, 218, 101], OperandSize::Qword)
}

