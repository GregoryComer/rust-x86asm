use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 218], OperandSize::Dword)
}

#[test]
fn ucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1674909916, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 36, 141, 220, 24, 213, 99], OperandSize::Dword)
}

#[test]
fn ucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 207], OperandSize::Qword)
}

#[test]
fn ucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 611718497, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 187, 97, 21, 118, 36], OperandSize::Qword)
}

