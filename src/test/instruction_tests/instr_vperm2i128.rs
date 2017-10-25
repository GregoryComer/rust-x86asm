use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vperm2i128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 70, 210, 82], OperandSize::Dword)
}

#[test]
fn vperm2i128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1790297437, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 70, 60, 181, 93, 197, 181, 106, 16], OperandSize::Dword)
}

#[test]
fn vperm2i128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 70, 207, 75], OperandSize::Qword)
}

#[test]
fn vperm2i128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1243282571, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 70, 188, 82, 139, 252, 26, 74, 110], OperandSize::Qword)
}

