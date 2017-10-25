use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 11, 210, 82], OperandSize::Dword)
}

#[test]
fn vroundsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 357984273, Some(OperandSize::Qword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 11, 156, 215, 17, 104, 86, 21, 22], OperandSize::Dword)
}

#[test]
fn vroundsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 11, 194, 80], OperandSize::Qword)
}

#[test]
fn vroundsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 11, 12, 177, 64], OperandSize::Qword)
}

