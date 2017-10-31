use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 12, 243], OperandSize::Dword)
}

#[test]
fn vpermilps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 12, 51], OperandSize::Dword)
}

#[test]
fn vpermilps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 12, 224], OperandSize::Qword)
}

#[test]
fn vpermilps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 12, 40], OperandSize::Qword)
}

#[test]
fn vpermilps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 12, 248], OperandSize::Dword)
}

#[test]
fn vpermilps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 809769669, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 12, 20, 125, 197, 26, 68, 48], OperandSize::Dword)
}

#[test]
fn vpermilps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 12, 216], OperandSize::Qword)
}

#[test]
fn vpermilps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 12, 60, 254], OperandSize::Qword)
}

#[test]
fn vpermilps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 12, 218], OperandSize::Dword)
}

#[test]
fn vpermilps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 12, 44, 86], OperandSize::Dword)
}

#[test]
fn vpermilps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1274057769, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 155, 12, 44, 149, 41, 148, 240, 75], OperandSize::Dword)
}

#[test]
fn vpermilps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 109, 132, 12, 249], OperandSize::Qword)
}

#[test]
fn vpermilps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 45, 138, 12, 60, 127], OperandSize::Qword)
}

#[test]
fn vpermilps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 122011388, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 85, 153, 12, 134, 252, 190, 69, 7], OperandSize::Qword)
}

#[test]
fn vpermilps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 12, 217], OperandSize::Dword)
}

#[test]
fn vpermilps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 12, 14], OperandSize::Dword)
}

#[test]
fn vpermilps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 186, 12, 4, 200], OperandSize::Dword)
}

#[test]
fn vpermilps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 77, 164, 12, 239], OperandSize::Qword)
}

#[test]
fn vpermilps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 101, 175, 12, 12, 198], OperandSize::Qword)
}

#[test]
fn vpermilps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RDI, 340074747, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 93, 183, 12, 175, 251, 32, 69, 20], OperandSize::Qword)
}

#[test]
fn vpermilps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 12, 213], OperandSize::Dword)
}

#[test]
fn vpermilps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 12, 36, 153], OperandSize::Dword)
}

#[test]
fn vpermilps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 778728880, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 12, 178, 176, 117, 106, 46], OperandSize::Dword)
}

#[test]
fn vpermilps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 101, 199, 12, 235], OperandSize::Qword)
}

#[test]
fn vpermilps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 85, 203, 12, 28, 139], OperandSize::Qword)
}

#[test]
fn vpermilps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 223, 12, 56], OperandSize::Qword)
}

#[test]
fn vpermilps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 248, 107], OperandSize::Dword)
}

#[test]
fn vpermilps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1522657649, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 145, 113, 233, 193, 90, 21], OperandSize::Dword)
}

#[test]
fn vpermilps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 197, 92], OperandSize::Qword)
}

#[test]
fn vpermilps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 62, 120], OperandSize::Qword)
}

#[test]
fn vpermilps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 236, 59], OperandSize::Dword)
}

#[test]
fn vpermilps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 51, 11], OperandSize::Dword)
}

#[test]
fn vpermilps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 227, 105], OperandSize::Qword)
}

#[test]
fn vpermilps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 60, 192, 66], OperandSize::Qword)
}

#[test]
fn vpermilps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 4, 219, 3], OperandSize::Dword)
}

#[test]
fn vpermilps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1794404658, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 4, 132, 119, 50, 113, 244, 106, 90], OperandSize::Dword)
}

#[test]
fn vpermilps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 332188666, Some(OperandSize::Dword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 154, 4, 180, 176, 250, 203, 204, 19, 126], OperandSize::Dword)
}

#[test]
fn vpermilps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 195, 125, 140, 4, 216, 113], OperandSize::Qword)
}

#[test]
fn vpermilps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1282693474, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 125, 139, 4, 188, 90, 98, 89, 116, 76, 66], OperandSize::Qword)
}

#[test]
fn vpermilps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1970616169, Some(OperandSize::Dword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 153, 4, 60, 181, 105, 55, 117, 117, 8], OperandSize::Qword)
}

#[test]
fn vpermilps_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 4, 221, 99], OperandSize::Dword)
}

#[test]
fn vpermilps_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1055297380, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 4, 36, 149, 100, 143, 230, 62, 38], OperandSize::Dword)
}

#[test]
fn vpermilps_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDX, 1001746663, Some(OperandSize::Dword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 4, 146, 231, 112, 181, 59, 6], OperandSize::Dword)
}

#[test]
fn vpermilps_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 125, 174, 4, 200, 83], OperandSize::Qword)
}

#[test]
fn vpermilps_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM24)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 125, 170, 4, 7, 63], OperandSize::Qword)
}

#[test]
fn vpermilps_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM23)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 186, 4, 56, 107], OperandSize::Qword)
}

#[test]
fn vpermilps_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 4, 209, 113], OperandSize::Dword)
}

#[test]
fn vpermilps_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 701646054, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 4, 153, 230, 68, 210, 41, 50], OperandSize::Dword)
}

#[test]
fn vpermilps_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 218, 4, 2, 60], OperandSize::Dword)
}

#[test]
fn vpermilps_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM11)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 125, 207, 4, 219, 125], OperandSize::Qword)
}

#[test]
fn vpermilps_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 125, 204, 4, 28, 211, 64], OperandSize::Qword)
}

#[test]
fn vpermilps_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 219, 4, 31, 54], OperandSize::Qword)
}

