use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn extractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 245, 70], OperandSize::Dword)
}

#[test]
fn extractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 630659303, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 60, 133, 231, 24, 151, 37, 106], OperandSize::Dword)
}

#[test]
fn extractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 223, 85], OperandSize::Qword)
}

#[test]
fn extractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 10, 62], OperandSize::Qword)
}

