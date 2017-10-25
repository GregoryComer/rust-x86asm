use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 252, 83], OperandSize::Word)
}

#[test]
fn pshufw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Qword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 1, 22], OperandSize::Word)
}

#[test]
fn pshufw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 220, 61], OperandSize::Dword)
}

#[test]
fn pshufw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 709530529, Some(OperandSize::Qword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 44, 149, 161, 147, 74, 42, 126], OperandSize::Dword)
}

#[test]
fn pshufw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 224, 81], OperandSize::Qword)
}

#[test]
fn pshufw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 52, 89, 51], OperandSize::Qword)
}

