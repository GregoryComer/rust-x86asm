use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 203], OperandSize::Dword)
}

#[test]
fn pmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1195440653, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 140, 159, 13, 250, 64, 71], OperandSize::Dword)
}

#[test]
fn pmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 245], OperandSize::Qword)
}

#[test]
fn pmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1320780330, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 36, 253, 42, 130, 185, 78], OperandSize::Qword)
}

