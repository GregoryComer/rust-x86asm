use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 187, 209], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 187, 0], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 187, 227], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 298011369, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 187, 132, 241, 233, 74, 195, 17], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 219, 187, 239], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 187, 4, 184], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 245, 159, 187, 196], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 129, 187, 46], OperandSize::Qword)
}

