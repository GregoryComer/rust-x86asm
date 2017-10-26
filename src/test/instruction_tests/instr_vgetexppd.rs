use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 66, 204], OperandSize::Dword)
}

#[test]
fn vgetexppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1875049187, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 66, 4, 189, 227, 250, 194, 111], OperandSize::Dword)
}

#[test]
fn vgetexppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 352617828, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 66, 28, 117, 100, 133, 4, 21], OperandSize::Dword)
}

#[test]
fn vgetexppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 253, 140, 66, 207], OperandSize::Qword)
}

#[test]
fn vgetexppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 139, 66, 44, 75], OperandSize::Qword)
}

#[test]
fn vgetexppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM22)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 158, 66, 49], OperandSize::Qword)
}

#[test]
fn vgetexppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 66, 221], OperandSize::Dword)
}

#[test]
fn vgetexppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 303959397, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 66, 172, 242, 101, 13, 30, 18], OperandSize::Dword)
}

#[test]
fn vgetexppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 66, 31], OperandSize::Dword)
}

#[test]
fn vgetexppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 175, 66, 243], OperandSize::Qword)
}

#[test]
fn vgetexppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 681865237, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 173, 66, 36, 197, 21, 112, 164, 40], OperandSize::Qword)
}

#[test]
fn vgetexppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM28)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 185, 66, 35], OperandSize::Qword)
}

#[test]
fn vgetexppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 157, 66, 205], OperandSize::Dword)
}

#[test]
fn vgetexppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 464591943, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 66, 60, 125, 71, 28, 177, 27], OperandSize::Dword)
}

#[test]
fn vgetexppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1358526814, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 66, 156, 81, 94, 121, 249, 80], OperandSize::Dword)
}

#[test]
fn vgetexppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 253, 155, 66, 248], OperandSize::Qword)
}

#[test]
fn vgetexppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 177823358, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 207, 66, 164, 217, 126, 94, 153, 10], OperandSize::Qword)
}

#[test]
fn vgetexppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1115042960, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 223, 66, 20, 213, 144, 52, 118, 66], OperandSize::Qword)
}

