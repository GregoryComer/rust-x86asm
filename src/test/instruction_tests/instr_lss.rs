use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 94, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 113, 94], OperandSize::Word)
}

#[test]
fn lss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 44, 130], OperandSize::Dword)
}

#[test]
fn lss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 52, 198], OperandSize::Qword)
}

#[test]
fn lss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1712702569, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 188, 120, 105, 196, 21, 102], OperandSize::Dword)
}

#[test]
fn lss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 500743486, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 140, 82, 62, 189, 216, 29], OperandSize::Qword)
}

#[test]
fn lss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 178, 60, 199], OperandSize::Qword)
}

