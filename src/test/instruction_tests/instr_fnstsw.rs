use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 78, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 121, 78], OperandSize::Word)
}

#[test]
fn fnstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 62], OperandSize::Dword)
}

#[test]
fn fnstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1992933300, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 213, 180, 191, 201, 118], OperandSize::Qword)
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

