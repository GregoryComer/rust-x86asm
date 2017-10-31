use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 222, 45, 219], OperandSize::Dword)
}

#[test]
fn vscalefsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 45, 4, 123], OperandSize::Dword)
}

#[test]
fn vscalefsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 221, 212, 45, 228], OperandSize::Qword)
}

#[test]
fn vscalefsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 181, 138, 45, 25], OperandSize::Qword)
}

