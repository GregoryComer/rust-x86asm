use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 205], OperandSize::Dword)
}

#[test]
fn comisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 138237377, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 12, 133, 193, 85, 61, 8], OperandSize::Dword)
}

#[test]
fn comisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 196], OperandSize::Qword)
}

#[test]
fn comisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1937732369, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 60, 253, 17, 115, 127, 115], OperandSize::Qword)
}

