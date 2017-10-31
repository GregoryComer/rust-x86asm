use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 240], OperandSize::Dword)
}

#[test]
fn ucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 50], OperandSize::Dword)
}

#[test]
fn ucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 225], OperandSize::Qword)
}

#[test]
fn ucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 170848490, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 36, 125, 234, 240, 46, 10], OperandSize::Qword)
}

