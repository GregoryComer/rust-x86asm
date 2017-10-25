use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 57, 213, 72], OperandSize::Dword)
}

#[test]
fn vextracti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 227402403, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 188, 209, 163, 226, 141, 13, 43], OperandSize::Dword)
}

#[test]
fn vextracti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 125, 207, 57, 200, 117], OperandSize::Qword)
}

#[test]
fn vextracti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectDisplaced(RCX, 1433372752, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 72, 57, 129, 80, 136, 111, 85, 118], OperandSize::Qword)
}

