use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 10, 213, 116], OperandSize::Dword)
}

#[test]
fn vroundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1584630701, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 10, 12, 77, 173, 139, 115, 94, 76], OperandSize::Dword)
}

#[test]
fn vroundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 10, 236, 39], OperandSize::Qword)
}

#[test]
fn vroundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1848035232, Some(OperandSize::Dword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 10, 60, 253, 160, 199, 38, 110, 25], OperandSize::Qword)
}

