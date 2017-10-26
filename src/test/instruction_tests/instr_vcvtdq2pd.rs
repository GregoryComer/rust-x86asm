use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 226], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1892857807, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 12, 197, 207, 183, 210, 112], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 219], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 7], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 220], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 1994361527, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 138, 183, 138, 223, 118], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 209], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 25], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 230, 218], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 818826499, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 230, 172, 79, 3, 77, 206, 48], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 126, 143, 230, 208], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 230, 2], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 230, 242], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 1202308933, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 230, 159, 69, 199, 169, 71], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 126, 170, 230, 253], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RCX, 1079070755, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 230, 137, 35, 80, 81, 64], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 230, 192], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 230, 43], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 126, 204, 230, 222], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 126, 206, 230, 23], OperandSize::Qword)
}

