use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 65, 221, 75], OperandSize::Dword)
}

#[test]
fn vdppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 65, 20, 87, 104], OperandSize::Dword)
}

#[test]
fn vdppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 65, 246, 21], OperandSize::Qword)
}

#[test]
fn vdppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1582901626, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 65, 12, 213, 122, 41, 89, 94, 96], OperandSize::Qword)
}

