use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 201, 57, 248, 84], OperandSize::Dword)
}

#[test]
fn vextracti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 17, 9], OperandSize::Dword)
}

#[test]
fn vextracti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM26)), operand2: Some(Direct(ZMM24)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 3, 125, 204, 57, 194, 1], OperandSize::Qword)
}

#[test]
fn vextracti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 52, 118, 23], OperandSize::Qword)
}

