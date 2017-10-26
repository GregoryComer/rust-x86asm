use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 9571, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 187, 99, 37], OperandSize::Word)
}

#[test]
fn fnstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(ECX, Two, 386383627, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 77, 11, 191, 7, 23], OperandSize::Dword)
}

#[test]
fn fnstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(RAX, Two, 980567054, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 69, 14, 68, 114, 58], OperandSize::Qword)
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

