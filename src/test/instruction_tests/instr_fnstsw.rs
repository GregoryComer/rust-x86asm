use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 61], OperandSize::Word)
}

#[test]
fn fnstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(EAX, Four, 784818391, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 133, 215, 96, 199, 46], OperandSize::Dword)
}

#[test]
fn fnstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 56], OperandSize::Qword)
}

#[test]
fn fnstsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Word)
}

#[test]
fn fnstsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Dword)
}

#[test]
fn fnstsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Qword)
}

