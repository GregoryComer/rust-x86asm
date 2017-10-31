use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 11, 192, 62], OperandSize::Dword)
}

#[test]
fn vroundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 11, 57, 59], OperandSize::Dword)
}

#[test]
fn vroundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 11, 202, 49], OperandSize::Qword)
}

#[test]
fn vroundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 455461045, Some(OperandSize::Qword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 11, 132, 118, 181, 200, 37, 27, 125], OperandSize::Qword)
}

