use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 66, 255], OperandSize::Dword)
}

#[test]
fn vgetexppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 590375766, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 66, 12, 253, 86, 107, 48, 35], OperandSize::Dword)
}

#[test]
fn vgetexppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 66, 54], OperandSize::Dword)
}

#[test]
fn vgetexppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 253, 140, 66, 228], OperandSize::Qword)
}

#[test]
fn vgetexppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 140, 66, 52, 123], OperandSize::Qword)
}

#[test]
fn vgetexppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 66, 44, 139], OperandSize::Qword)
}

#[test]
fn vgetexppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 66, 205], OperandSize::Dword)
}

#[test]
fn vgetexppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 66, 62], OperandSize::Dword)
}

#[test]
fn vgetexppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 950874761, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 66, 180, 214, 137, 50, 173, 56], OperandSize::Dword)
}

#[test]
fn vgetexppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 169, 66, 194], OperandSize::Qword)
}

#[test]
fn vgetexppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1192067549, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 169, 66, 12, 69, 221, 129, 13, 71], OperandSize::Qword)
}

#[test]
fn vgetexppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 189, 66, 19], OperandSize::Qword)
}

#[test]
fn vgetexppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 157, 66, 243], OperandSize::Dword)
}

#[test]
fn vgetexppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1800090490, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 66, 44, 189, 122, 51, 75, 107], OperandSize::Dword)
}

#[test]
fn vgetexppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 941915773, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 66, 60, 85, 125, 126, 36, 56], OperandSize::Dword)
}

#[test]
fn vgetexppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 253, 158, 66, 239], OperandSize::Qword)
}

#[test]
fn vgetexppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RCX, 518353089, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 201, 66, 185, 193, 112, 229, 30], OperandSize::Qword)
}

#[test]
fn vgetexppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectDisplaced(RBX, 260562187, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 222, 66, 163, 11, 221, 135, 15], OperandSize::Qword)
}

