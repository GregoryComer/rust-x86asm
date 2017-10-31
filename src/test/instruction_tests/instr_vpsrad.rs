use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 231, 109], OperandSize::Dword)
}

#[test]
fn vpsrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 114, 230, 115], OperandSize::Qword)
}

#[test]
fn vpsrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 226, 110], OperandSize::Dword)
}

#[test]
fn vpsrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 114, 229, 57], OperandSize::Qword)
}

#[test]
fn vpsrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 114, 224, 92], OperandSize::Dword)
}

#[test]
fn vpsrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 114, 35, 52], OperandSize::Dword)
}

#[test]
fn vpsrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 562603660, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 155, 114, 36, 197, 140, 166, 136, 33, 72], OperandSize::Dword)
}

#[test]
fn vpsrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 109, 138, 114, 225, 49], OperandSize::Qword)
}

#[test]
fn vpsrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 526763073, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 114, 167, 65, 196, 101, 31, 27], OperandSize::Qword)
}

#[test]
fn vpsrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 940622074, Some(OperandSize::Dword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 157, 114, 164, 241, 250, 192, 16, 56, 48], OperandSize::Qword)
}

#[test]
fn vpsrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 114, 227, 127], OperandSize::Dword)
}

#[test]
fn vpsrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 114, 36, 185, 114], OperandSize::Dword)
}

#[test]
fn vpsrad_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1838972690, Some(OperandSize::Dword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 189, 114, 164, 90, 18, 127, 156, 109, 84], OperandSize::Dword)
}

#[test]
fn vpsrad_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 85, 175, 114, 229, 43], OperandSize::Qword)
}

#[test]
fn vpsrad_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 174, 114, 38, 109], OperandSize::Qword)
}

#[test]
fn vpsrad_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1428640999, Some(OperandSize::Dword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 45, 188, 114, 164, 137, 231, 84, 39, 85, 123], OperandSize::Qword)
}

#[test]
fn vpsrad_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 114, 231, 52], OperandSize::Dword)
}

#[test]
fn vpsrad_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1246658105, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 114, 164, 223, 57, 126, 78, 74, 6], OperandSize::Dword)
}

#[test]
fn vpsrad_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ESI, 614229323, Some(OperandSize::Dword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 219, 114, 166, 75, 101, 156, 36, 78], OperandSize::Dword)
}

#[test]
fn vpsrad_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 101, 195, 114, 225, 115], OperandSize::Qword)
}

#[test]
fn vpsrad_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectDisplaced(RBX, 1774259451, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 29, 204, 114, 163, 251, 12, 193, 105, 73], OperandSize::Qword)
}

#[test]
fn vpsrad_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 143494127, Some(OperandSize::Dword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 219, 114, 36, 133, 239, 139, 141, 8, 119], OperandSize::Qword)
}

#[test]
fn vpsrad_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 226, 224], OperandSize::Dword)
}

#[test]
fn vpsrad_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 239912350, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 226, 36, 149, 158, 197, 76, 14], OperandSize::Dword)
}

#[test]
fn vpsrad_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 226, 232], OperandSize::Qword)
}

#[test]
fn vpsrad_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 909121125, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 226, 160, 101, 22, 48, 54], OperandSize::Qword)
}

#[test]
fn vpsrad_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 226, 224], OperandSize::Dword)
}

#[test]
fn vpsrad_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 226, 59], OperandSize::Dword)
}

#[test]
fn vpsrad_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 226, 204], OperandSize::Qword)
}

#[test]
fn vpsrad_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 735322644, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 226, 180, 153, 20, 34, 212, 43], OperandSize::Qword)
}

#[test]
fn vpsrad_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 226, 234], OperandSize::Dword)
}

#[test]
fn vpsrad_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 226, 55], OperandSize::Dword)
}

#[test]
fn vpsrad_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 85, 129, 226, 226], OperandSize::Qword)
}

#[test]
fn vpsrad_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 332609319, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 109, 135, 226, 164, 119, 39, 55, 211, 19], OperandSize::Qword)
}

#[test]
fn vpsrad_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 226, 216], OperandSize::Dword)
}

#[test]
fn vpsrad_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 1600439073, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 226, 130, 33, 195, 100, 95], OperandSize::Dword)
}

#[test]
fn vpsrad_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 45, 169, 226, 230], OperandSize::Qword)
}

#[test]
fn vpsrad_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RAX, 616684776, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 53, 172, 226, 144, 232, 220, 193, 36], OperandSize::Qword)
}

#[test]
fn vpsrad_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 226, 225], OperandSize::Dword)
}

#[test]
fn vpsrad_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1991122475, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 226, 36, 93, 43, 30, 174, 118], OperandSize::Dword)
}

#[test]
fn vpsrad_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 85, 193, 226, 226], OperandSize::Qword)
}

#[test]
fn vpsrad_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectDisplaced(RBX, 1171060475, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 13, 205, 226, 179, 251, 246, 204, 69], OperandSize::Qword)
}

