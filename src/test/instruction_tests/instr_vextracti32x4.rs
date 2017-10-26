use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 57, 251, 9], OperandSize::Dword)
}

#[test]
fn vextracti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 52, 136, 108], OperandSize::Dword)
}

#[test]
fn vextracti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 204, 57, 210, 21], OperandSize::Qword)
}

#[test]
fn vextracti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 72, 57, 51, 13], OperandSize::Qword)
}

