use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 154, 81, 217, 72], OperandSize::Dword)
}

#[test]
fn vrangess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1367679414, Some(OperandSize::Dword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 117, 140, 81, 4, 93, 182, 33, 133, 81, 84], OperandSize::Dword)
}

#[test]
fn vrangess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM12)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 77, 154, 81, 228, 81], OperandSize::Qword)
}

#[test]
fn vrangess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 2074353680, Some(OperandSize::Dword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 93, 129, 81, 188, 152, 16, 32, 164, 123, 104], OperandSize::Qword)
}

