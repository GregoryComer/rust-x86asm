use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 86, 229], OperandSize::Dword)
}

#[test]
fn vorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1260317004, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 86, 153, 76, 233, 30, 75], OperandSize::Dword)
}

#[test]
fn vorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 86, 211], OperandSize::Qword)
}

#[test]
fn vorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 181227301, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 12, 117, 37, 79, 205, 10], OperandSize::Qword)
}

#[test]
fn vorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 86, 218], OperandSize::Dword)
}

#[test]
fn vorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 86, 62], OperandSize::Dword)
}

#[test]
fn vorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 86, 211], OperandSize::Qword)
}

#[test]
fn vorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1995366483, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 86, 52, 245, 83, 224, 238, 118], OperandSize::Qword)
}

#[test]
fn vorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 86, 200], OperandSize::Dword)
}

#[test]
fn vorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1789172730, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 86, 36, 253, 250, 155, 164, 106], OperandSize::Dword)
}

#[test]
fn vorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 159, 86, 52, 88], OperandSize::Dword)
}

#[test]
fn vorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 181, 132, 86, 238], OperandSize::Qword)
}

#[test]
fn vorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1952134865, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 129, 86, 44, 221, 209, 54, 91, 116], OperandSize::Qword)
}

#[test]
fn vorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1314359761, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 205, 159, 86, 4, 85, 209, 137, 87, 78], OperandSize::Qword)
}

#[test]
fn vorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 86, 197], OperandSize::Dword)
}

#[test]
fn vorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 501646429, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 173, 86, 12, 253, 93, 132, 230, 29], OperandSize::Dword)
}

#[test]
fn vorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 190, 86, 41], OperandSize::Dword)
}

#[test]
fn vorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 197, 170, 86, 230], OperandSize::Qword)
}

#[test]
fn vorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RSI, 527993725, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 165, 161, 86, 158, 125, 139, 120, 31], OperandSize::Qword)
}

#[test]
fn vorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 2066535953, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 181, 181, 86, 188, 177, 17, 214, 44, 123], OperandSize::Qword)
}

#[test]
fn vorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 86, 210], OperandSize::Dword)
}

#[test]
fn vorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 557595622, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 86, 60, 221, 230, 59, 60, 33], OperandSize::Dword)
}

#[test]
fn vorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 748696893, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 223, 86, 148, 73, 61, 53, 160, 44], OperandSize::Dword)
}

#[test]
fn vorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 173, 195, 86, 203], OperandSize::Qword)
}

#[test]
fn vorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDX, 1655299405, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 213, 196, 86, 130, 77, 221, 169, 98], OperandSize::Qword)
}

#[test]
fn vorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 189, 223, 86, 62], OperandSize::Qword)
}

