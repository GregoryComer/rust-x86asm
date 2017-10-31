use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 217, 38], OperandSize::Dword)
}

#[test]
fn vextractf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1118045188, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 20, 245, 4, 4, 164, 66, 19], OperandSize::Dword)
}

#[test]
fn vextractf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 223, 57], OperandSize::Qword)
}

#[test]
fn vextractf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 44, 113, 42], OperandSize::Qword)
}

