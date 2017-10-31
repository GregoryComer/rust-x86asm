use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 66, 253], OperandSize::Dword)
}

#[test]
fn vgetexppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 66, 38], OperandSize::Dword)
}

#[test]
fn vgetexppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 142345946, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 66, 52, 181, 218, 6, 124, 8], OperandSize::Dword)
}

#[test]
fn vgetexppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 253, 137, 66, 227], OperandSize::Qword)
}

#[test]
fn vgetexppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 781052367, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 142, 66, 36, 213, 207, 233, 141, 46], OperandSize::Qword)
}

#[test]
fn vgetexppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1922611524, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 159, 66, 52, 197, 68, 185, 152, 114], OperandSize::Qword)
}

#[test]
fn vgetexppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 66, 221], OperandSize::Dword)
}

#[test]
fn vgetexppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 890491598, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 66, 4, 93, 206, 210, 19, 53], OperandSize::Dword)
}

#[test]
fn vgetexppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EBX, 1180931926, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 186, 66, 187, 86, 151, 99, 70], OperandSize::Dword)
}

#[test]
fn vgetexppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 253, 170, 66, 221], OperandSize::Qword)
}

#[test]
fn vgetexppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 2010420894, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 66, 12, 149, 158, 150, 212, 119], OperandSize::Qword)
}

#[test]
fn vgetexppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM22)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 187, 66, 51], OperandSize::Qword)
}

#[test]
fn vgetexppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 156, 66, 223], OperandSize::Dword)
}

#[test]
fn vgetexppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 66, 48], OperandSize::Dword)
}

#[test]
fn vgetexppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 66, 62], OperandSize::Dword)
}

#[test]
fn vgetexppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 253, 155, 66, 217], OperandSize::Qword)
}

#[test]
fn vgetexppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 204, 66, 20, 122], OperandSize::Qword)
}

#[test]
fn vgetexppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 2085064350, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 222, 66, 60, 189, 158, 142, 71, 124], OperandSize::Qword)
}

