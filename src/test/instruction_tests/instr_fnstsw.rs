use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 233, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 184, 233, 0], OperandSize::Word)
}

#[test]
fn fnstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1262691123, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 189, 51, 35, 67, 75], OperandSize::Dword)
}

#[test]
fn fnstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 86], OperandSize::Qword)
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

