use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 235], OperandSize::Word)
}

#[test]
fn verw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Memory(27030, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 46, 150, 105], OperandSize::Word)
}

#[test]
fn verw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 236], OperandSize::Dword)
}

#[test]
fn verw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectDisplaced(ESI, 1406306844, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 174, 28, 138, 210, 83], OperandSize::Dword)
}

#[test]
fn verw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 238], OperandSize::Qword)
}

#[test]
fn verw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 44, 183], OperandSize::Qword)
}

