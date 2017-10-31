use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1067079517, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 36, 125, 93, 87, 154, 63], OperandSize::Dword)
}

#[test]
fn vbroadcastf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1112733305, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 60, 149, 121, 246, 82, 66], OperandSize::Qword)
}

