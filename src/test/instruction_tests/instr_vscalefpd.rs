use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 44, 255], OperandSize::Dword)
}

#[test]
fn vscalefpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1386556988, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 44, 12, 149, 60, 46, 165, 82], OperandSize::Dword)
}

#[test]
fn vscalefpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 157, 44, 9], OperandSize::Dword)
}

#[test]
fn vscalefpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 133, 137, 44, 230], OperandSize::Qword)
}

#[test]
fn vscalefpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 61645829, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 129, 44, 4, 205, 5, 164, 172, 3], OperandSize::Qword)
}

#[test]
fn vscalefpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 556475715, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 141, 151, 44, 180, 210, 67, 37, 43, 33], OperandSize::Qword)
}

#[test]
fn vscalefpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 44, 212], OperandSize::Dword)
}

#[test]
fn vscalefpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 44, 1], OperandSize::Dword)
}

#[test]
fn vscalefpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 190, 44, 44, 199], OperandSize::Dword)
}

#[test]
fn vscalefpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 149, 162, 44, 227], OperandSize::Qword)
}

#[test]
fn vscalefpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 169, 44, 60, 251], OperandSize::Qword)
}

#[test]
fn vscalefpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1447516253, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 141, 179, 44, 172, 127, 93, 88, 71, 86], OperandSize::Qword)
}

#[test]
fn vscalefpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 188, 44, 200], OperandSize::Dword)
}

#[test]
fn vscalefpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 44, 52, 201], OperandSize::Dword)
}

#[test]
fn vscalefpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1822106023, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 217, 44, 156, 206, 167, 33, 155, 108], OperandSize::Dword)
}

#[test]
fn vscalefpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 159, 44, 197], OperandSize::Qword)
}

#[test]
fn vscalefpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 372548402, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 157, 203, 44, 156, 152, 50, 163, 52, 22], OperandSize::Qword)
}

#[test]
fn vscalefpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RAX, 1429079230, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 219, 44, 152, 190, 4, 46, 85], OperandSize::Qword)
}

