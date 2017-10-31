use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 238], OperandSize::Dword)
}

#[test]
fn pminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 4, 208], OperandSize::Dword)
}

#[test]
fn pminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 254], OperandSize::Qword)
}

#[test]
fn pminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1788328734, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 164, 195, 30, 187, 151, 106], OperandSize::Qword)
}

