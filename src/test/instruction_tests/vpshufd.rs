use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 236, 65], OperandSize::Dword)
}

fn vpshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EAX, 1836283481, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 168, 89, 118, 115, 109, 16], OperandSize::Dword)
}

fn vpshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 206, 36], OperandSize::Qword)
}

fn vpshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 14, 104], OperandSize::Qword)
}

fn vpshufd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 229, 91], OperandSize::Dword)
}

fn vpshufd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 521842559, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 36, 245, 127, 175, 26, 31, 10], OperandSize::Dword)
}

fn vpshufd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 249, 103], OperandSize::Qword)
}

fn vpshufd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 251555832, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 60, 125, 248, 111, 254, 14, 34], OperandSize::Qword)
}

fn vpshufd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 112, 219, 35], OperandSize::Dword)
}

fn vpshufd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 112, 57, 21], OperandSize::Dword)
}

fn vpshufd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 26816664, Some(OperandSize::Dword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 158, 112, 44, 117, 152, 48, 153, 1, 32], OperandSize::Dword)
}

fn vpshufd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 125, 141, 112, 227, 76], OperandSize::Qword)
}

fn vpshufd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1671391930, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 125, 139, 112, 12, 181, 186, 106, 159, 99, 24], OperandSize::Qword)
}

fn vpshufd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 397651609, Some(OperandSize::Dword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 112, 44, 141, 153, 174, 179, 23, 99], OperandSize::Qword)
}

fn vpshufd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 112, 242, 76], OperandSize::Dword)
}

fn vpshufd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 112, 34, 88], OperandSize::Dword)
}

fn vpshufd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 187, 112, 58, 7], OperandSize::Dword)
}

fn vpshufd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 171, 112, 237, 124], OperandSize::Qword)
}

fn vpshufd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectDisplaced(RSI, 1360975058, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 175, 112, 166, 210, 212, 30, 81, 48], OperandSize::Qword)
}

fn vpshufd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM25)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 125, 190, 112, 8, 32], OperandSize::Qword)
}

fn vpshufd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 112, 249, 69], OperandSize::Dword)
}

fn vpshufd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EBX, 490390216, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 112, 131, 200, 194, 58, 29, 122], OperandSize::Dword)
}

fn vpshufd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 218, 112, 35, 70], OperandSize::Dword)
}

fn vpshufd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 125, 203, 112, 249, 66], OperandSize::Qword)
}

fn vpshufd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1063019316, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 201, 112, 36, 85, 52, 99, 92, 63, 109], OperandSize::Qword)
}

fn vpshufd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1560205392, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 125, 217, 112, 156, 190, 80, 216, 254, 92, 53], OperandSize::Qword)
}

