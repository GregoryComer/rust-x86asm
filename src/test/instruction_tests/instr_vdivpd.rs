use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 94, 252], OperandSize::Dword)
}

#[test]
fn vdivpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 94, 36, 193], OperandSize::Dword)
}

#[test]
fn vdivpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 218], OperandSize::Qword)
}

#[test]
fn vdivpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1467073025, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 94, 140, 70, 1, 194, 113, 87], OperandSize::Qword)
}

#[test]
fn vdivpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 240], OperandSize::Dword)
}

#[test]
fn vdivpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 527411030, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 94, 164, 143, 86, 167, 111, 31], OperandSize::Dword)
}

#[test]
fn vdivpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 246], OperandSize::Qword)
}

#[test]
fn vdivpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 94, 60, 194], OperandSize::Qword)
}

#[test]
fn vdivpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 94, 201], OperandSize::Dword)
}

#[test]
fn vdivpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 94, 36, 129], OperandSize::Dword)
}

#[test]
fn vdivpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 156, 94, 33], OperandSize::Dword)
}

#[test]
fn vdivpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 221, 135, 94, 244], OperandSize::Qword)
}

#[test]
fn vdivpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1432505095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 181, 135, 94, 44, 149, 7, 75, 98, 85], OperandSize::Qword)
}

#[test]
fn vdivpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 155, 94, 28, 75], OperandSize::Qword)
}

#[test]
fn vdivpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 175, 94, 236], OperandSize::Dword)
}

#[test]
fn vdivpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 10847994, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 174, 94, 36, 141, 250, 134, 165, 0], OperandSize::Dword)
}

#[test]
fn vdivpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 186, 94, 20, 251], OperandSize::Dword)
}

#[test]
fn vdivpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 165, 161, 94, 234], OperandSize::Qword)
}

#[test]
fn vdivpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 993070894, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 173, 174, 94, 132, 144, 46, 15, 49, 59], OperandSize::Qword)
}

#[test]
fn vdivpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 966615856, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 205, 186, 94, 146, 48, 99, 157, 57], OperandSize::Qword)
}

#[test]
fn vdivpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 252, 94, 216], OperandSize::Dword)
}

#[test]
fn vdivpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 82570418, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 94, 185, 178, 236, 235, 4], OperandSize::Dword)
}

#[test]
fn vdivpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 421201916, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 218, 94, 52, 221, 252, 7, 27, 25], OperandSize::Dword)
}

#[test]
fn vdivpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 213, 209, 94, 212], OperandSize::Qword)
}

#[test]
fn vdivpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 141, 205, 94, 30], OperandSize::Qword)
}

#[test]
fn vdivpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 229, 209, 94, 24], OperandSize::Qword)
}

