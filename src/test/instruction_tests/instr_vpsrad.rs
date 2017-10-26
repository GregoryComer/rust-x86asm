use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 114, 225, 79], OperandSize::Dword)
}

#[test]
fn vpsrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 114, 231, 5], OperandSize::Qword)
}

#[test]
fn vpsrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 114, 230, 93], OperandSize::Dword)
}

#[test]
fn vpsrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 114, 228, 62], OperandSize::Qword)
}

#[test]
fn vpsrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 114, 231, 98], OperandSize::Dword)
}

#[test]
fn vpsrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 918141045, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 114, 36, 197, 117, 184, 185, 54, 115], OperandSize::Dword)
}

#[test]
fn vpsrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 153, 114, 33, 1], OperandSize::Dword)
}

#[test]
fn vpsrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 21, 141, 114, 226, 74], OperandSize::Qword)
}

#[test]
fn vpsrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 29, 131, 114, 32, 15], OperandSize::Qword)
}

#[test]
fn vpsrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 398225151, Some(OperandSize::Dword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 159, 114, 164, 90, 255, 110, 188, 23, 4], OperandSize::Qword)
}

#[test]
fn vpsrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 114, 228, 77], OperandSize::Dword)
}

#[test]
fn vpsrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1527375543, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 114, 36, 125, 183, 230, 9, 91, 116], OperandSize::Dword)
}

#[test]
fn vpsrad_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 188, 114, 36, 71, 28], OperandSize::Dword)
}

#[test]
fn vpsrad_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM27)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 21, 172, 114, 227, 59], OperandSize::Qword)
}

#[test]
fn vpsrad_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 114, 35, 41], OperandSize::Qword)
}

#[test]
fn vpsrad_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1478103153, Some(OperandSize::Dword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 181, 114, 164, 95, 113, 16, 26, 88, 91], OperandSize::Qword)
}

#[test]
fn vpsrad_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 114, 230, 72], OperandSize::Dword)
}

#[test]
fn vpsrad_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 114, 39, 105], OperandSize::Dword)
}

#[test]
fn vpsrad_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 108908830, Some(OperandSize::Dword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 217, 114, 36, 117, 30, 209, 125, 6, 47], OperandSize::Dword)
}

#[test]
fn vpsrad_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 53, 196, 114, 224, 120], OperandSize::Qword)
}

#[test]
fn vpsrad_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 387515628, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 114, 36, 253, 236, 4, 25, 23, 102], OperandSize::Qword)
}

#[test]
fn vpsrad_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 215, 114, 38, 32], OperandSize::Qword)
}

#[test]
fn vpsrad_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 226, 234], OperandSize::Dword)
}

#[test]
fn vpsrad_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 226, 25], OperandSize::Dword)
}

#[test]
fn vpsrad_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 226, 223], OperandSize::Qword)
}

#[test]
fn vpsrad_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1227438577, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 226, 172, 209, 241, 57, 41, 73], OperandSize::Qword)
}

#[test]
fn vpsrad_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 226, 202], OperandSize::Dword)
}

#[test]
fn vpsrad_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 226, 28, 134], OperandSize::Dword)
}

#[test]
fn vpsrad_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 226, 235], OperandSize::Qword)
}

#[test]
fn vpsrad_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 226, 36, 86], OperandSize::Qword)
}

#[test]
fn vpsrad_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 226, 248], OperandSize::Dword)
}

#[test]
fn vpsrad_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 2126438144, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 140, 226, 154, 0, 223, 190, 126], OperandSize::Dword)
}

#[test]
fn vpsrad_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 109, 140, 226, 221], OperandSize::Qword)
}

#[test]
fn vpsrad_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 45, 138, 226, 30], OperandSize::Qword)
}

#[test]
fn vpsrad_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 226, 223], OperandSize::Dword)
}

#[test]
fn vpsrad_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 226, 23], OperandSize::Dword)
}

#[test]
fn vpsrad_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 101, 170, 226, 238], OperandSize::Qword)
}

#[test]
fn vpsrad_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 80632766, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 162, 226, 180, 185, 190, 91, 206, 4], OperandSize::Qword)
}

#[test]
fn vpsrad_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 226, 241], OperandSize::Dword)
}

#[test]
fn vpsrad_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 226, 52, 73], OperandSize::Dword)
}

#[test]
fn vpsrad_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 29, 197, 226, 208], OperandSize::Qword)
}

#[test]
fn vpsrad_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 5, 202, 226, 15], OperandSize::Qword)
}

