use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 549876497, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 91, 180, 150, 17, 115, 198, 32], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X8, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 2086234235, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 91, 36, 197, 123, 104, 89, 124], OperandSize::Qword)
}

