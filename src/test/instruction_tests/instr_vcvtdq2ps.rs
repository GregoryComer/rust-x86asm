use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 236], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 1219995415, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 174, 23, 167, 183, 72], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 245], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 1635661942, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 169, 118, 56, 126, 97], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 242], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 51], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 199], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 46], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 91, 211], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 2007433643, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 91, 4, 149, 171, 1, 167, 119], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 124, 137, 91, 237], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 464440248, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 143, 91, 172, 145, 184, 203, 174, 27], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 91, 202], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EAX, 1617886419, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 91, 128, 211, 252, 110, 96], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 124, 170, 91, 198], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1051167035, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 174, 91, 20, 157, 59, 137, 167, 62], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 153, 91, 248], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 166606552, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 91, 28, 197, 216, 54, 238, 9], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 124, 217, 91, 212], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 258969420, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 207, 91, 28, 221, 76, 143, 111, 15], OperandSize::Qword)
}

