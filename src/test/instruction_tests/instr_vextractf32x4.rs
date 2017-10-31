use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 25, 222, 37], OperandSize::Dword)
}

#[test]
fn vextractf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectDisplaced(EBX, 692665195, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 163, 107, 59, 73, 41, 8], OperandSize::Dword)
}

#[test]
fn vextractf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM16)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 125, 202, 25, 232, 11], OperandSize::Qword)
}

#[test]
fn vextractf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 28, 86, 83], OperandSize::Qword)
}

