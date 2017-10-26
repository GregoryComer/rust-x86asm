use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 191, 241], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 657449872, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 191, 4, 77, 144, 227, 47, 39], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 191, 244], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 191, 34], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 222, 191, 240], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1878013979, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 191, 60, 149, 27, 56, 240, 111], OperandSize::Dword)
}

#[test]
fn vfnmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 197, 189, 191, 207], OperandSize::Qword)
}

#[test]
fn vfnmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 129, 191, 12, 243], OperandSize::Qword)
}

