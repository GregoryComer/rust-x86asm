use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 251, 45, 221], OperandSize::Dword)
}

#[test]
fn vscalefsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 45, 12, 78], OperandSize::Dword)
}

#[test]
fn vscalefsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 141, 156, 45, 214], OperandSize::Qword)
}

#[test]
fn vscalefsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 189, 138, 45, 12, 118], OperandSize::Qword)
}

