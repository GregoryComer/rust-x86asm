use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 57, 202, 17], OperandSize::Dword)
}

#[test]
fn vextracti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledDisplaced(ESI, Four, 166820868, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 20, 181, 4, 124, 241, 9, 18], OperandSize::Dword)
}

#[test]
fn vextracti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM20)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 51, 125, 202, 57, 252, 123], OperandSize::Qword)
}

#[test]
fn vextracti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1955367514, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 115, 125, 72, 57, 172, 128, 90, 138, 140, 116, 106], OperandSize::Qword)
}

