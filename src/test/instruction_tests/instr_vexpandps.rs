use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 136, 249], OperandSize::Dword)
}

#[test]
fn vexpandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1479817727, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 136, 4, 125, 255, 57, 52, 88], OperandSize::Dword)
}

#[test]
fn vexpandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 138, 136, 204], OperandSize::Qword)
}

#[test]
fn vexpandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 136, 28, 118], OperandSize::Qword)
}

#[test]
fn vexpandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 136, 250], OperandSize::Dword)
}

#[test]
fn vexpandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 136, 52, 64], OperandSize::Dword)
}

#[test]
fn vexpandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 125, 172, 136, 192], OperandSize::Qword)
}

#[test]
fn vexpandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 136, 12, 79], OperandSize::Qword)
}

#[test]
fn vexpandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 136, 241], OperandSize::Dword)
}

#[test]
fn vexpandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDX, 1247075451, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 136, 170, 123, 220, 84, 74], OperandSize::Dword)
}

#[test]
fn vexpandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 125, 203, 136, 237], OperandSize::Qword)
}

#[test]
fn vexpandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 60635365, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 136, 12, 85, 229, 56, 157, 3], OperandSize::Qword)
}

