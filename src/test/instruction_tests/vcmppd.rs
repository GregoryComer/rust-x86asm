use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 194, 216, 114], OperandSize::Dword)
}

fn vcmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 194, 36, 195, 4], OperandSize::Dword)
}

fn vcmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 194, 209, 49], OperandSize::Qword)
}

fn vcmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 922937065, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 194, 169, 233, 230, 2, 55, 6], OperandSize::Qword)
}

fn vcmppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 194, 223, 3], OperandSize::Dword)
}

fn vcmppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 1397281846, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 194, 178, 54, 212, 72, 83, 99], OperandSize::Dword)
}

fn vcmppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 194, 223, 74], OperandSize::Qword)
}

fn vcmppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1945618926, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 194, 12, 149, 238, 201, 247, 115, 85], OperandSize::Qword)
}

fn vcmppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 11, 194, 250, 114], OperandSize::Dword)
}

fn vcmppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 11, 194, 12, 139, 73], OperandSize::Dword)
}

fn vcmppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1793379334, Some(OperandSize::Qword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 28, 194, 154, 6, 204, 228, 106, 85], OperandSize::Dword)
}

fn vcmppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 133, 11, 194, 243, 5], OperandSize::Qword)
}

fn vcmppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RAX, 940921674, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 189, 4, 194, 136, 74, 83, 21, 56, 29], OperandSize::Qword)
}

fn vcmppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 19, 194, 28, 187, 106], OperandSize::Qword)
}

fn vcmppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 45, 194, 207, 44], OperandSize::Dword)
}

fn vcmppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1664231079, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 41, 194, 28, 181, 167, 38, 50, 99, 125], OperandSize::Dword)
}

fn vcmppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1388811270, Some(OperandSize::Qword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 57, 194, 188, 182, 6, 148, 199, 82, 31], OperandSize::Dword)
}

fn vcmppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM8)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 221, 33, 194, 232, 43], OperandSize::Qword)
}

fn vcmppd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1582013142, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 133, 33, 194, 188, 145, 214, 154, 75, 94, 57], OperandSize::Qword)
}

fn vcmppd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RDX, 766358044, Some(OperandSize::Qword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 165, 62, 194, 170, 28, 178, 173, 45, 25], OperandSize::Qword)
}

fn vcmppd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 27, 194, 250, 29], OperandSize::Dword)
}

fn vcmppd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2080492024, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 79, 194, 12, 245, 248, 201, 1, 124, 125], OperandSize::Dword)
}

fn vcmppd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 956038080, Some(OperandSize::Qword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 91, 194, 188, 137, 192, 251, 251, 56, 29], OperandSize::Dword)
}

fn vcmppd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM9)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 229, 20, 194, 249, 89], OperandSize::Qword)
}

fn vcmppd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1321342172, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 68, 194, 164, 203, 220, 20, 194, 78, 78], OperandSize::Qword)
}

fn vcmppd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 90, 194, 20, 217, 76], OperandSize::Qword)
}

