use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 11, 221, 118], OperandSize::Dword)
}

#[test]
fn vroundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 306045054, Some(OperandSize::Qword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 11, 132, 198, 126, 224, 61, 18, 110], OperandSize::Dword)
}

#[test]
fn vroundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 11, 201, 21], OperandSize::Qword)
}

#[test]
fn vroundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 433326267, Some(OperandSize::Qword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 11, 20, 125, 187, 8, 212, 25, 56], OperandSize::Qword)
}

