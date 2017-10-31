use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 189, 205], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 189, 20, 185], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 189, 202], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 189, 0], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 191, 189, 237], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 280586071, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 189, 156, 210, 87, 103, 185, 16], OperandSize::Dword)
}

#[test]
fn vfnmadd231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 181, 244, 189, 248], OperandSize::Qword)
}

#[test]
fn vfnmadd231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 173, 131, 189, 3], OperandSize::Qword)
}

