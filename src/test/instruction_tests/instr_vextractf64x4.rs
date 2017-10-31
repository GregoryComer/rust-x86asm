use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 27, 194, 27], OperandSize::Dword)
}

#[test]
fn vextractf64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 30, 65], OperandSize::Dword)
}

#[test]
fn vextractf64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM30)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 253, 202, 27, 230, 100], OperandSize::Qword)
}

#[test]
fn vextractf64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM10)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 115, 253, 72, 27, 20, 70, 11], OperandSize::Qword)
}

