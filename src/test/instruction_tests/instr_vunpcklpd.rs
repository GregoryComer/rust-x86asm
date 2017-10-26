use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 20, 194], OperandSize::Dword)
}

#[test]
fn vunpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1078964354, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 20, 148, 158, 130, 176, 79, 64], OperandSize::Dword)
}

#[test]
fn vunpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 20, 217], OperandSize::Qword)
}

#[test]
fn vunpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 20, 28, 191], OperandSize::Qword)
}

#[test]
fn vunpcklpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 20, 197], OperandSize::Dword)
}

#[test]
fn vunpcklpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 20, 41], OperandSize::Dword)
}

#[test]
fn vunpcklpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 20, 245], OperandSize::Qword)
}

#[test]
fn vunpcklpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 20, 40], OperandSize::Qword)
}

#[test]
fn vunpcklpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 20, 237], OperandSize::Dword)
}

#[test]
fn vunpcklpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1082643390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 20, 155, 190, 211, 135, 64], OperandSize::Dword)
}

#[test]
fn vunpcklpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 159, 20, 36, 120], OperandSize::Dword)
}

#[test]
fn vunpcklpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 173, 129, 20, 205], OperandSize::Qword)
}

#[test]
fn vunpcklpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1035122906, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 129, 20, 12, 149, 218, 184, 178, 61], OperandSize::Qword)
}

#[test]
fn vunpcklpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RAX, 2140207580, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 189, 154, 20, 160, 220, 249, 144, 127], OperandSize::Qword)
}

#[test]
fn vunpcklpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 20, 203], OperandSize::Dword)
}

#[test]
fn vunpcklpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 20, 52, 193], OperandSize::Dword)
}

#[test]
fn vunpcklpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 190, 20, 6], OperandSize::Dword)
}

#[test]
fn vunpcklpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 205, 169, 20, 215], OperandSize::Qword)
}

#[test]
fn vunpcklpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDI, 789780263, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 197, 173, 20, 143, 39, 23, 19, 47], OperandSize::Qword)
}

#[test]
fn vunpcklpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 173, 186, 20, 39], OperandSize::Qword)
}

#[test]
fn vunpcklpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 20, 233], OperandSize::Dword)
}

#[test]
fn vunpcklpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 1070065003, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 20, 129, 107, 229, 199, 63], OperandSize::Dword)
}

#[test]
fn vunpcklpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 684956903, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 218, 20, 172, 118, 231, 156, 211, 40], OperandSize::Dword)
}

#[test]
fn vunpcklpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 237, 197, 20, 229], OperandSize::Qword)
}

#[test]
fn vunpcklpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 205, 20, 10], OperandSize::Qword)
}

#[test]
fn vunpcklpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 181, 215, 20, 4, 192], OperandSize::Qword)
}

