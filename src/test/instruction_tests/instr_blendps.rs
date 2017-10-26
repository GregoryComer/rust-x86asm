use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 218, 35], OperandSize::Dword)
}

#[test]
fn blendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 52, 67, 93], OperandSize::Dword)
}

#[test]
fn blendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 211, 39], OperandSize::Qword)
}

#[test]
fn blendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1367456358, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 36, 245, 102, 186, 129, 81, 70], OperandSize::Qword)
}

