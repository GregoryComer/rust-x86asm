use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 247], OperandSize::Dword)
}

#[test]
fn pminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 20, 127], OperandSize::Dword)
}

#[test]
fn pminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 207], OperandSize::Qword)
}

#[test]
fn pminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 845249828, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 156, 123, 36, 125, 97, 50], OperandSize::Qword)
}

