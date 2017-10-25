use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 114, 227, 71], OperandSize::Dword)
}

fn vpsrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 114, 229, 100], OperandSize::Qword)
}

fn vpsrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 114, 227, 44], OperandSize::Dword)
}

fn vpsrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 114, 225, 38], OperandSize::Qword)
}

fn vpsrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 114, 226, 89], OperandSize::Dword)
}

fn vpsrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 731017702, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 114, 36, 189, 230, 113, 146, 43, 96], OperandSize::Dword)
}

fn vpsrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 1299458729, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 114, 167, 169, 42, 116, 77, 73], OperandSize::Dword)
}

fn vpsrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 135, 114, 225, 89], OperandSize::Qword)
}

fn vpsrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 130, 114, 36, 138, 124], OperandSize::Qword)
}

fn vpsrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 358619159, Some(OperandSize::Dword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 158, 114, 164, 113, 23, 24, 96, 21, 85], OperandSize::Qword)
}

fn vpsrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 114, 230, 45], OperandSize::Dword)
}

fn vpsrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1478290639, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 114, 36, 93, 207, 236, 28, 88, 98], OperandSize::Dword)
}

fn vpsrad_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 191, 114, 35, 3], OperandSize::Dword)
}

fn vpsrad_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM24)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 61, 170, 114, 224, 40], OperandSize::Qword)
}

fn vpsrad_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 45, 164, 114, 36, 122, 95], OperandSize::Qword)
}

fn vpsrad_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 21, 180, 114, 36, 210, 81], OperandSize::Qword)
}

fn vpsrad_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 114, 228, 125], OperandSize::Dword)
}

fn vpsrad_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 114, 34, 126], OperandSize::Dword)
}

fn vpsrad_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 219, 114, 36, 143, 22], OperandSize::Dword)
}

fn vpsrad_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 93, 207, 114, 231, 113], OperandSize::Qword)
}

fn vpsrad_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 195, 114, 34, 86], OperandSize::Qword)
}

fn vpsrad_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 13, 213, 114, 36, 139, 72], OperandSize::Qword)
}

fn vpsrad_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 226, 242], OperandSize::Dword)
}

fn vpsrad_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 226, 52, 211], OperandSize::Dword)
}

fn vpsrad_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 226, 239], OperandSize::Qword)
}

fn vpsrad_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1542385645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 226, 164, 179, 237, 239, 238, 91], OperandSize::Qword)
}

fn vpsrad_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 226, 250], OperandSize::Dword)
}

fn vpsrad_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 876448489, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 226, 162, 233, 138, 61, 52], OperandSize::Dword)
}

fn vpsrad_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 226, 194], OperandSize::Qword)
}

fn vpsrad_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 226, 60, 178], OperandSize::Qword)
}

fn vpsrad_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 226, 223], OperandSize::Dword)
}

fn vpsrad_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 226, 52, 144], OperandSize::Dword)
}

fn vpsrad_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 109, 140, 226, 224], OperandSize::Qword)
}

fn vpsrad_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RSI, 1289505437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 61, 137, 226, 142, 157, 74, 220, 76], OperandSize::Qword)
}

fn vpsrad_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 226, 207], OperandSize::Dword)
}

fn vpsrad_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 1626760349, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 226, 131, 157, 100, 246, 96], OperandSize::Dword)
}

fn vpsrad_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 69, 174, 226, 225], OperandSize::Qword)
}

fn vpsrad_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 69, 161, 226, 28, 246], OperandSize::Qword)
}

fn vpsrad_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 201, 226, 235], OperandSize::Dword)
}

fn vpsrad_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 792419369, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 226, 169, 41, 92, 59, 47], OperandSize::Dword)
}

fn vpsrad_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 45, 193, 226, 234], OperandSize::Qword)
}

fn vpsrad_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 199, 226, 20, 191], OperandSize::Qword)
}

