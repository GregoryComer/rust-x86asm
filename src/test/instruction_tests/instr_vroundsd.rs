use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 11, 223, 1], OperandSize::Dword)
}

#[test]
fn vroundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 348346244, Some(OperandSize::Qword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 11, 162, 132, 87, 195, 20, 77], OperandSize::Dword)
}

#[test]
fn vroundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 11, 238, 86], OperandSize::Qword)
}

#[test]
fn vroundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 11, 4, 243, 72], OperandSize::Qword)
}

