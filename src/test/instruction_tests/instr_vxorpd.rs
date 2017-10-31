use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 87, 203], OperandSize::Dword)
}

#[test]
fn vxorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 87, 52, 182], OperandSize::Dword)
}

#[test]
fn vxorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 87, 254], OperandSize::Qword)
}

#[test]
fn vxorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 87, 36, 118], OperandSize::Qword)
}

#[test]
fn vxorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 87, 213], OperandSize::Dword)
}

#[test]
fn vxorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1341220254, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 87, 140, 88, 158, 101, 241, 79], OperandSize::Dword)
}

#[test]
fn vxorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 87, 201], OperandSize::Qword)
}

#[test]
fn vxorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 87, 42], OperandSize::Qword)
}

#[test]
fn vxorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 87, 230], OperandSize::Dword)
}

#[test]
fn vxorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 87, 41], OperandSize::Dword)
}

#[test]
fn vxorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 87, 12, 65], OperandSize::Dword)
}

#[test]
fn vxorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 221, 131, 87, 254], OperandSize::Qword)
}

#[test]
fn vxorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 189, 142, 87, 54], OperandSize::Qword)
}

#[test]
fn vxorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 189, 153, 87, 44, 72], OperandSize::Qword)
}

#[test]
fn vxorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 172, 87, 208], OperandSize::Dword)
}

#[test]
fn vxorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1241520833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 172, 87, 180, 90, 193, 26, 0, 74], OperandSize::Dword)
}

#[test]
fn vxorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 2058665803, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 87, 182, 75, 191, 180, 122], OperandSize::Dword)
}

#[test]
fn vxorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 221, 174, 87, 215], OperandSize::Qword)
}

#[test]
fn vxorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RCX, 1513252841, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 173, 171, 87, 137, 233, 103, 50, 90], OperandSize::Qword)
}

#[test]
fn vxorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1381393334, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 165, 191, 87, 28, 197, 182, 99, 86, 82], OperandSize::Qword)
}

#[test]
fn vxorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 87, 202], OperandSize::Dword)
}

#[test]
fn vxorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDX, 1541274178, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 204, 87, 146, 66, 250, 221, 91], OperandSize::Dword)
}

#[test]
fn vxorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 160036198, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 219, 87, 44, 197, 102, 245, 137, 9], OperandSize::Dword)
}

#[test]
fn vxorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 205, 194, 87, 218], OperandSize::Qword)
}

#[test]
fn vxorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 87, 41], OperandSize::Qword)
}

#[test]
fn vxorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 213, 212, 87, 28, 78], OperandSize::Qword)
}

