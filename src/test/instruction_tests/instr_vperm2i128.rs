use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2i128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 70, 221, 54], OperandSize::Dword)
}

#[test]
fn vperm2i128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 70, 44, 223, 21], OperandSize::Dword)
}

#[test]
fn vperm2i128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 70, 222, 43], OperandSize::Qword)
}

#[test]
fn vperm2i128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1128043792, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 70, 140, 123, 16, 149, 60, 67, 116], OperandSize::Qword)
}

