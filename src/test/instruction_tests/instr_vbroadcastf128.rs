use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 668676777, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 180, 129, 169, 50, 219, 39], OperandSize::Dword)
}

#[test]
fn vbroadcastf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDX, 2139451330, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 170, 194, 111, 133, 127], OperandSize::Qword)
}

