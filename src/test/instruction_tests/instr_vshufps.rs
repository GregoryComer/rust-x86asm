use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 198, 204, 103], OperandSize::Dword)
}

#[test]
fn vshufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 198, 11, 99], OperandSize::Dword)
}

#[test]
fn vshufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 198, 233, 75], OperandSize::Qword)
}

#[test]
fn vshufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 198, 52, 78, 92], OperandSize::Qword)
}

#[test]
fn vshufps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 198, 215, 17], OperandSize::Dword)
}

#[test]
fn vshufps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 647338507, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 198, 191, 11, 154, 149, 38, 61], OperandSize::Dword)
}

#[test]
fn vshufps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 198, 227, 97], OperandSize::Qword)
}

#[test]
fn vshufps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 1579933901, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 198, 155, 205, 224, 43, 94, 0], OperandSize::Qword)
}

#[test]
fn vshufps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 140, 198, 192, 80], OperandSize::Dword)
}

#[test]
fn vshufps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 141, 198, 32, 65], OperandSize::Dword)
}

#[test]
fn vshufps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 158, 198, 22, 102], OperandSize::Dword)
}

#[test]
fn vshufps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 60, 138, 198, 231, 72], OperandSize::Qword)
}

#[test]
fn vshufps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RCX, 1064064180, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 28, 143, 198, 129, 180, 84, 108, 63, 112], OperandSize::Qword)
}

#[test]
fn vshufps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 12, 146, 198, 35, 35], OperandSize::Qword)
}

#[test]
fn vshufps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 175, 198, 204, 28], OperandSize::Dword)
}

#[test]
fn vshufps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 1583456057, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 198, 167, 57, 159, 97, 94, 75], OperandSize::Dword)
}

#[test]
fn vshufps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 190, 198, 62, 42], OperandSize::Dword)
}

#[test]
fn vshufps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM24)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 116, 175, 198, 216, 110], OperandSize::Qword)
}

#[test]
fn vshufps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 44, 166, 198, 20, 195, 108], OperandSize::Qword)
}

#[test]
fn vshufps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 170652968, Some(OperandSize::Dword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 180, 198, 12, 253, 40, 245, 43, 10, 48], OperandSize::Qword)
}

#[test]
fn vshufps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 204, 198, 239, 98], OperandSize::Dword)
}

#[test]
fn vshufps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 649503239, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 201, 198, 132, 64, 7, 162, 182, 38, 117], OperandSize::Dword)
}

#[test]
fn vshufps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 442121950, Some(OperandSize::Dword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 217, 198, 172, 144, 222, 62, 90, 26, 37], OperandSize::Dword)
}

#[test]
fn vshufps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 100, 195, 198, 239, 14], OperandSize::Qword)
}

#[test]
fn vshufps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1808034688, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 116, 193, 198, 140, 153, 128, 107, 196, 107, 69], OperandSize::Qword)
}

#[test]
fn vshufps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 2129843077, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 100, 214, 198, 132, 209, 133, 211, 242, 126, 82], OperandSize::Qword)
}

