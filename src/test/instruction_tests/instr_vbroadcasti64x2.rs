use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 90, 63], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 171, 90, 36, 151], OperandSize::Qword)
}

#[test]
fn vbroadcasti64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 90, 34], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 765251664, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 90, 164, 182, 80, 208, 156, 45], OperandSize::Qword)
}

