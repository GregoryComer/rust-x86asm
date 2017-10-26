use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 158, 45, 214], OperandSize::Dword)
}

#[test]
fn vscalefsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1248813878, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 45, 147, 54, 99, 111, 74], OperandSize::Dword)
}

#[test]
fn vscalefsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 141, 157, 45, 203], OperandSize::Qword)
}

#[test]
fn vscalefsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 165, 129, 45, 33], OperandSize::Qword)
}

