use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 236], OperandSize::Word)
}

#[test]
fn verw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 40], OperandSize::Word)
}

#[test]
fn verw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 233], OperandSize::Dword)
}

#[test]
fn verw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledDisplaced(EDI, Four, 677279707, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 44, 189, 219, 119, 94, 40], OperandSize::Dword)
}

#[test]
fn verw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 235], OperandSize::Qword)
}

#[test]
fn verw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1828285816, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 172, 79, 120, 109, 249, 108], OperandSize::Qword)
}

