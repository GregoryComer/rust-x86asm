use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 59, 255, 82], OperandSize::Dword)
}

#[test]
fn vextracti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1879077148, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 52, 245, 28, 113, 0, 112, 81], OperandSize::Dword)
}

#[test]
fn vextracti64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM11)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 67, 253, 203, 59, 227, 79], OperandSize::Qword)
}

#[test]
fn vextracti64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledDisplaced(RCX, Two, 474303424, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 28, 77, 192, 75, 69, 28, 108], OperandSize::Qword)
}

