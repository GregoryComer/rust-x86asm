use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 203, 218], OperandSize::Dword)
}

#[test]
fn vrcp28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1102516272, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 203, 52, 85, 48, 16, 183, 65], OperandSize::Dword)
}

#[test]
fn vrcp28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 197, 147, 203, 196], OperandSize::Qword)
}

#[test]
fn vrcp28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1190625850, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 203, 180, 94, 58, 130, 247, 70], OperandSize::Qword)
}

