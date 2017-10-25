use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 243, 104], OperandSize::Dword)
}

#[test]
fn vextracti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1605482289, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 156, 114, 49, 183, 177, 95, 99], OperandSize::Dword)
}

#[test]
fn vextracti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 234, 42], OperandSize::Qword)
}

#[test]
fn vextracti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1452692300, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 20, 69, 76, 83, 150, 86, 97], OperandSize::Qword)
}

