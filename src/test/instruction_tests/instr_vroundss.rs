use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 10, 219, 85], OperandSize::Dword)
}

#[test]
fn vroundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 10, 47, 115], OperandSize::Dword)
}

#[test]
fn vroundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 235, 61], OperandSize::Qword)
}

#[test]
fn vroundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1332534472, Some(OperandSize::Dword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 10, 12, 197, 200, 220, 108, 79, 2], OperandSize::Qword)
}

