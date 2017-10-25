use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 198], OperandSize::Word)
}

#[test]
fn sldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 29152, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 128, 224, 113], OperandSize::Word)
}

#[test]
fn sldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 193], OperandSize::Dword)
}

#[test]
fn sldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 6], OperandSize::Dword)
}

#[test]
fn sldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 193], OperandSize::Qword)
}

#[test]
fn sldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1779232657, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 132, 150, 145, 239, 12, 106], OperandSize::Qword)
}

#[test]
fn sldt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 0, 196], OperandSize::Qword)
}

#[test]
fn sldt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SLDT, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 6], OperandSize::Qword)
}

