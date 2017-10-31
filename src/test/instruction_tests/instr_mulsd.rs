use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 221], OperandSize::Dword)
}

#[test]
fn mulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1926757552, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 4, 181, 176, 252, 215, 114], OperandSize::Dword)
}

#[test]
fn mulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 202], OperandSize::Qword)
}

#[test]
fn mulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1175361367, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 52, 125, 87, 151, 14, 70], OperandSize::Qword)
}

