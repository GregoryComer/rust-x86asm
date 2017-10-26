use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 230, 109], OperandSize::Dword)
}

#[test]
fn pcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 681858321, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 188, 78, 17, 85, 164, 40, 60], OperandSize::Dword)
}

#[test]
fn pcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 244, 2], OperandSize::Qword)
}

#[test]
fn pcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1690358042, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 156, 186, 26, 209, 192, 100, 7], OperandSize::Qword)
}

