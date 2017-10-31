use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 59, 222, 62], OperandSize::Dword)
}

#[test]
fn vextracti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 519482609, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 156, 222, 241, 172, 246, 30, 114], OperandSize::Dword)
}

#[test]
fn vextracti64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM21)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 206, 59, 233, 77], OperandSize::Qword)
}

#[test]
fn vextracti64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectDisplaced(RAX, 1352772833, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 176, 225, 172, 161, 80, 1], OperandSize::Qword)
}

