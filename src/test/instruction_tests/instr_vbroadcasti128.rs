use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1462318091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 4, 125, 11, 52, 41, 87], OperandSize::Dword)
}

#[test]
fn vbroadcasti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 469647658, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 28, 245, 42, 65, 254, 27], OperandSize::Qword)
}

