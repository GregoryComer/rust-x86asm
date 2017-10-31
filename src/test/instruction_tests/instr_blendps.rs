use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 236, 73], OperandSize::Dword)
}

#[test]
fn blendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 33, 50], OperandSize::Dword)
}

#[test]
fn blendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 255, 77], OperandSize::Qword)
}

#[test]
fn blendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1947277855, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 52, 117, 31, 26, 17, 116, 10], OperandSize::Qword)
}

