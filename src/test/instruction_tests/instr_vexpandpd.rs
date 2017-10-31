use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 136, 218], OperandSize::Dword)
}

#[test]
fn vexpandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 517368929, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 136, 137, 97, 108, 214, 30], OperandSize::Dword)
}

#[test]
fn vexpandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 253, 138, 136, 199], OperandSize::Qword)
}

#[test]
fn vexpandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM18)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 143, 136, 22], OperandSize::Qword)
}

#[test]
fn vexpandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 136, 204], OperandSize::Dword)
}

#[test]
fn vexpandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 136, 11], OperandSize::Dword)
}

#[test]
fn vexpandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 253, 174, 136, 206], OperandSize::Qword)
}

#[test]
fn vexpandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 150947312, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 171, 136, 180, 143, 240, 69, 255, 8], OperandSize::Qword)
}

#[test]
fn vexpandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 136, 247], OperandSize::Dword)
}

#[test]
fn vexpandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 4784687, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 136, 44, 85, 47, 2, 73, 0], OperandSize::Dword)
}

#[test]
fn vexpandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 253, 203, 136, 192], OperandSize::Qword)
}

#[test]
fn vexpandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1784695073, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 205, 136, 12, 221, 33, 73, 96, 106], OperandSize::Qword)
}

