use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 249, 81], OperandSize::Dword)
}

#[test]
fn vextractf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectDisplaced(EDX, 314856830, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 154, 126, 85, 196, 18, 51], OperandSize::Dword)
}

#[test]
fn vextractf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 195, 125], OperandSize::Qword)
}

#[test]
fn vextractf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectDisplaced(RAX, 1320459718, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 184, 198, 157, 180, 78, 60], OperandSize::Qword)
}

