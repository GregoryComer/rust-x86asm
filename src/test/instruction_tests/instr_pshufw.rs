use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 229, 117], OperandSize::Word)
}

#[test]
fn pshufw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(BP, 2031, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 190, 239, 7, 71], OperandSize::Word)
}

#[test]
fn pshufw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 223, 2], OperandSize::Dword)
}

#[test]
fn pshufw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 10, 0], OperandSize::Dword)
}

#[test]
fn pshufw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 203, 126], OperandSize::Qword)
}

#[test]
fn pshufw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 20, 94, 82], OperandSize::Qword)
}

