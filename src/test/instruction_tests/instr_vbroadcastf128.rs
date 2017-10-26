use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 908868704, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 20, 117, 96, 60, 44, 54], OperandSize::Dword)
}

#[test]
fn vbroadcastf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 2034791089, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 52, 117, 177, 114, 72, 121], OperandSize::Qword)
}

