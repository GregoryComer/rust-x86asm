use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 252, 73], OperandSize::Dword)
}

#[test]
fn vextracti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectDisplaced(EBX, 476206578, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 171, 242, 85, 98, 28, 78], OperandSize::Dword)
}

#[test]
fn vextracti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 200, 119], OperandSize::Qword)
}

#[test]
fn vextracti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectDisplaced(RDI, 1588423394, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 175, 226, 106, 173, 94, 5], OperandSize::Qword)
}

