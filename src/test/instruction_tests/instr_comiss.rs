use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 211], OperandSize::Dword)
}

#[test]
fn comiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 63], OperandSize::Dword)
}

#[test]
fn comiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 222], OperandSize::Qword)
}

#[test]
fn comiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 489469108, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 52, 189, 180, 180, 44, 29], OperandSize::Qword)
}

