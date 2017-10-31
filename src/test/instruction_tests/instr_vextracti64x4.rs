use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 59, 198, 62], OperandSize::Dword)
}

#[test]
fn vextracti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 497892555, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 140, 206, 203, 60, 173, 29, 124], OperandSize::Dword)
}

#[test]
fn vextracti64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 253, 205, 59, 219, 114], OperandSize::Qword)
}

#[test]
fn vextracti64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledDisplaced(RSI, Two, 344668028, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM21)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 253, 72, 59, 44, 117, 124, 55, 139, 20, 121], OperandSize::Qword)
}

