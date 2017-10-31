use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 187, 197], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 187, 60, 217], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 187, 199], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 756078207, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 187, 148, 88, 127, 214, 16, 45], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 188, 187, 195], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 187, 30], OperandSize::Dword)
}

#[test]
fn vfmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 221, 243, 187, 235], OperandSize::Qword)
}

#[test]
fn vfmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 221, 138, 187, 4, 115], OperandSize::Qword)
}

