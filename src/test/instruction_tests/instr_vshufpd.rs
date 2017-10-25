use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 198, 226, 82], OperandSize::Dword)
}

#[test]
fn vshufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1298976234, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 198, 12, 213, 234, 205, 108, 77, 17], OperandSize::Dword)
}

#[test]
fn vshufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 198, 227, 6], OperandSize::Qword)
}

#[test]
fn vshufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 198, 22, 18], OperandSize::Qword)
}

#[test]
fn vshufpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 198, 250, 68], OperandSize::Dword)
}

#[test]
fn vshufpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2026545326, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 198, 36, 205, 174, 160, 202, 120, 18], OperandSize::Dword)
}

#[test]
fn vshufpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 198, 214, 32], OperandSize::Qword)
}

#[test]
fn vshufpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1216731077, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 198, 60, 149, 197, 215, 133, 72, 33], OperandSize::Qword)
}

#[test]
fn vshufpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 198, 199, 13], OperandSize::Dword)
}

#[test]
fn vshufpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 198, 0, 41], OperandSize::Dword)
}

#[test]
fn vshufpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 155, 198, 19, 21], OperandSize::Dword)
}

#[test]
fn vshufpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 237, 140, 198, 218, 66], OperandSize::Qword)
}

#[test]
fn vshufpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 205, 132, 198, 12, 88, 100], OperandSize::Qword)
}

#[test]
fn vshufpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 189, 159, 198, 57, 82], OperandSize::Qword)
}

#[test]
fn vshufpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 171, 198, 242, 82], OperandSize::Dword)
}

#[test]
fn vshufpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 198, 50, 30], OperandSize::Dword)
}

#[test]
fn vshufpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 185, 198, 33, 45], OperandSize::Dword)
}

#[test]
fn vshufpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 173, 174, 198, 195, 86], OperandSize::Qword)
}

#[test]
fn vshufpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 175, 198, 60, 182, 87], OperandSize::Qword)
}

#[test]
fn vshufpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 173, 183, 198, 57, 74], OperandSize::Qword)
}

#[test]
fn vshufpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 198, 196, 4], OperandSize::Dword)
}

#[test]
fn vshufpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EDX, 156974440, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 198, 130, 104, 61, 91, 9, 24], OperandSize::Dword)
}

#[test]
fn vshufpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1376336704, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 222, 198, 132, 142, 64, 59, 9, 82, 0], OperandSize::Dword)
}

#[test]
fn vshufpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 197, 198, 254, 94], OperandSize::Qword)
}

#[test]
fn vshufpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RCX, 311137572, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 221, 194, 198, 185, 36, 149, 139, 18, 127], OperandSize::Qword)
}

#[test]
fn vshufpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 141, 219, 198, 4, 216, 5], OperandSize::Qword)
}

