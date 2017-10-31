use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 205, 72], OperandSize::Dword)
}

#[test]
fn vshufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1022717886, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 198, 44, 141, 190, 111, 245, 60, 64], OperandSize::Dword)
}

#[test]
fn vshufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 198, 230, 67], OperandSize::Qword)
}

#[test]
fn vshufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1044394052, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 170, 68, 48, 64, 62, 17], OperandSize::Qword)
}

#[test]
fn vshufps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 198, 212, 23], OperandSize::Dword)
}

#[test]
fn vshufps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1390587373, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 198, 44, 181, 237, 173, 226, 82, 103], OperandSize::Dword)
}

#[test]
fn vshufps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 198, 242, 34], OperandSize::Qword)
}

#[test]
fn vshufps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 1669395051, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 198, 185, 107, 242, 128, 99, 67], OperandSize::Qword)
}

#[test]
fn vshufps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 140, 198, 254, 119], OperandSize::Dword)
}

#[test]
fn vshufps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1861642797, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 141, 198, 172, 246, 45, 106, 246, 110, 36], OperandSize::Dword)
}

#[test]
fn vshufps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 658923385, Some(OperandSize::Dword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 154, 198, 182, 121, 95, 70, 39, 104], OperandSize::Dword)
}

#[test]
fn vshufps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM12)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 124, 133, 198, 212, 113], OperandSize::Qword)
}

#[test]
fn vshufps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 92, 138, 198, 15, 49], OperandSize::Qword)
}

#[test]
fn vshufps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1652472515, Some(OperandSize::Dword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 36, 148, 198, 52, 221, 195, 186, 126, 98, 42], OperandSize::Qword)
}

#[test]
fn vshufps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 198, 238, 27], OperandSize::Dword)
}

#[test]
fn vshufps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 164113129, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 174, 198, 129, 233, 42, 200, 9, 94], OperandSize::Dword)
}

#[test]
fn vshufps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 188, 198, 14, 18], OperandSize::Dword)
}

#[test]
fn vshufps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM23)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 4, 172, 198, 207, 37], OperandSize::Qword)
}

#[test]
fn vshufps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 44, 174, 198, 44, 222, 117], OperandSize::Qword)
}

#[test]
fn vshufps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 52, 178, 198, 49, 44], OperandSize::Qword)
}

#[test]
fn vshufps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 204, 198, 241, 117], OperandSize::Dword)
}

#[test]
fn vshufps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 492660481, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 204, 198, 148, 177, 1, 103, 93, 29, 27], OperandSize::Dword)
}

#[test]
fn vshufps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 210507669, Some(OperandSize::Dword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 218, 198, 182, 149, 23, 140, 12, 36], OperandSize::Dword)
}

#[test]
fn vshufps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 60, 207, 198, 227, 55], OperandSize::Qword)
}

#[test]
fn vshufps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 116, 204, 198, 48, 26], OperandSize::Qword)
}

#[test]
fn vshufps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 499567645, Some(OperandSize::Dword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 20, 218, 198, 148, 159, 29, 204, 198, 29, 117], OperandSize::Qword)
}

