use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 230, 245], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 140, 230, 32], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 254, 140, 230, 199], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM31)), operand2: Some(IndirectDisplaced(RSI, 231967715, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 254, 139, 230, 190, 227, 139, 211, 13], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 230, 218], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EAX, 1134770392, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 230, 168, 216, 56, 163, 67], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 254, 172, 230, 231], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 622987947, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 230, 4, 141, 171, 10, 34, 37], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 186, 230, 242], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 203, 230, 18], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 254, 190, 230, 254], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 202, 230, 28, 114], OperandSize::Qword)
}

