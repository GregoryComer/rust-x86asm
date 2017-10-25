use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ltr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 220], OperandSize::Word)
}

#[test]
fn ltr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28], OperandSize::Word)
}

#[test]
fn ltr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 220], OperandSize::Dword)
}

#[test]
fn ltr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 24], OperandSize::Dword)
}

#[test]
fn ltr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 219], OperandSize::Qword)
}

#[test]
fn ltr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1282503579, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 181, 155, 115, 113, 76], OperandSize::Qword)
}

