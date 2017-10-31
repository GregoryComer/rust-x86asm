use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 192], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 1236105652, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 150, 180, 121, 173, 73], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 206], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RBX, 1979103546, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 171, 58, 185, 246, 117], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 217], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 28, 177], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 233], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 23], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 90, 223], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 45449359, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 90, 158, 143, 128, 181, 2], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 137, 90, 209], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 613324313, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 90, 177, 25, 150, 142, 36], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 90, 252], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 90, 4, 150], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 124, 170, 90, 236], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 175, 90, 44, 194], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 154, 90, 214], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 90, 4, 72], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 124, 153, 90, 243], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 388108357, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 206, 90, 172, 78, 69, 16, 34, 23], OperandSize::Qword)
}

