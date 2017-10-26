use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 25, 247, 113], OperandSize::Dword)
}

#[test]
fn vextractf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectDisplaced(ESI, 1642435324, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 158, 252, 146, 229, 97, 57], OperandSize::Dword)
}

#[test]
fn vextractf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 204, 25, 242, 16], OperandSize::Qword)
}

#[test]
fn vextractf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 2102615385, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 52, 197, 89, 93, 83, 125, 0], OperandSize::Qword)
}

