use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 20, 213], OperandSize::Dword)
}

#[test]
fn vunpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 628400437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 20, 4, 157, 53, 161, 116, 37], OperandSize::Dword)
}

#[test]
fn vunpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 20, 238], OperandSize::Qword)
}

#[test]
fn vunpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 1357301431, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 20, 143, 183, 198, 230, 80], OperandSize::Qword)
}

#[test]
fn vunpcklpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 20, 201], OperandSize::Dword)
}

#[test]
fn vunpcklpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 20, 36, 118], OperandSize::Dword)
}

#[test]
fn vunpcklpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 20, 237], OperandSize::Qword)
}

#[test]
fn vunpcklpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 20, 55], OperandSize::Qword)
}

#[test]
fn vunpcklpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 20, 250], OperandSize::Dword)
}

#[test]
fn vunpcklpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 20, 51], OperandSize::Dword)
}

#[test]
fn vunpcklpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 156, 20, 18], OperandSize::Dword)
}

#[test]
fn vunpcklpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 229, 129, 20, 227], OperandSize::Qword)
}

#[test]
fn vunpcklpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1011044669, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 165, 131, 20, 44, 77, 61, 81, 67, 60], OperandSize::Qword)
}

#[test]
fn vunpcklpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RDX, 1410311740, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 149, 147, 20, 146, 60, 166, 15, 84], OperandSize::Qword)
}

#[test]
fn vunpcklpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 20, 254], OperandSize::Dword)
}

#[test]
fn vunpcklpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 20, 8], OperandSize::Dword)
}

#[test]
fn vunpcklpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 794389723, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 190, 20, 185, 219, 108, 89, 47], OperandSize::Dword)
}

#[test]
fn vunpcklpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 165, 161, 20, 246], OperandSize::Qword)
}

#[test]
fn vunpcklpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 181, 166, 20, 60, 145], OperandSize::Qword)
}

#[test]
fn vunpcklpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 221, 180, 20, 25], OperandSize::Qword)
}

#[test]
fn vunpcklpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 20, 212], OperandSize::Dword)
}

#[test]
fn vunpcklpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 207, 20, 4, 177], OperandSize::Dword)
}

#[test]
fn vunpcklpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1641836812, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 217, 20, 180, 153, 12, 113, 220, 97], OperandSize::Dword)
}

#[test]
fn vunpcklpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 229, 202, 20, 217], OperandSize::Qword)
}

#[test]
fn vunpcklpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1510329328, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 20, 156, 136, 240, 203, 5, 90], OperandSize::Qword)
}

#[test]
fn vunpcklpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 285060831, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 237, 222, 20, 180, 216, 223, 174, 253, 16], OperandSize::Qword)
}

