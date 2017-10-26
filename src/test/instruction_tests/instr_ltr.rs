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
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectDisplaced(SI, 40, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 92, 40], OperandSize::Word)
}

#[test]
fn ltr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 217], OperandSize::Dword)
}

#[test]
fn ltr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 30], OperandSize::Dword)
}

#[test]
fn ltr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 220], OperandSize::Qword)
}

#[test]
fn ltr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 186], OperandSize::Qword)
}

