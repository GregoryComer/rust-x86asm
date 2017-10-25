use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 20, 240], OperandSize::Dword)
}

#[test]
fn vunpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 671624924, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 20, 52, 149, 220, 46, 8, 40], OperandSize::Dword)
}

#[test]
fn vunpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 20, 212], OperandSize::Qword)
}

#[test]
fn vunpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1513979156, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 20, 20, 245, 20, 125, 61, 90], OperandSize::Qword)
}

#[test]
fn vunpcklpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 20, 254], OperandSize::Dword)
}

#[test]
fn vunpcklpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 1046534149, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 20, 142, 5, 216, 96, 62], OperandSize::Dword)
}

#[test]
fn vunpcklpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 20, 212], OperandSize::Qword)
}

#[test]
fn vunpcklpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 421148959, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 20, 164, 123, 31, 57, 26, 25], OperandSize::Qword)
}

#[test]
fn vunpcklpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 20, 239], OperandSize::Dword)
}

#[test]
fn vunpcklpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 20, 6], OperandSize::Dword)
}

#[test]
fn vunpcklpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1256588894, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 157, 20, 28, 245, 94, 6, 230, 74], OperandSize::Dword)
}

#[test]
fn vunpcklpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 149, 130, 20, 241], OperandSize::Qword)
}

#[test]
fn vunpcklpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1235820324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 149, 133, 20, 44, 77, 36, 31, 169, 73], OperandSize::Qword)
}

#[test]
fn vunpcklpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 148, 20, 60, 112], OperandSize::Qword)
}

#[test]
fn vunpcklpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 20, 219], OperandSize::Dword)
}

#[test]
fn vunpcklpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 260686197, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 175, 20, 186, 117, 193, 137, 15], OperandSize::Dword)
}

#[test]
fn vunpcklpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 88262497, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 185, 20, 132, 94, 97, 199, 66, 5], OperandSize::Dword)
}

#[test]
fn vunpcklpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 181, 175, 20, 245], OperandSize::Qword)
}

#[test]
fn vunpcklpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RSI, 333597264, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 133, 172, 20, 142, 80, 74, 226, 19], OperandSize::Qword)
}

#[test]
fn vunpcklpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 173, 189, 20, 12, 70], OperandSize::Qword)
}

#[test]
fn vunpcklpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 20, 216], OperandSize::Dword)
}

#[test]
fn vunpcklpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 1368608283, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 20, 179, 27, 78, 147, 81], OperandSize::Dword)
}

#[test]
fn vunpcklpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1403952557, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 217, 20, 156, 177, 173, 157, 174, 83], OperandSize::Dword)
}

#[test]
fn vunpcklpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 213, 203, 20, 198], OperandSize::Qword)
}

#[test]
fn vunpcklpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1760499960, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 181, 203, 20, 156, 241, 248, 24, 239, 104], OperandSize::Qword)
}

#[test]
fn vunpcklpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RDI, 1477251325, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 149, 218, 20, 143, 253, 16, 13, 88], OperandSize::Qword)
}

