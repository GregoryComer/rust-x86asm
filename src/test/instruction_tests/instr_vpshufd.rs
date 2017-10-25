use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 250, 0], OperandSize::Dword)
}

#[test]
fn vpshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 123878901, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 148, 219, 245, 61, 98, 7, 86], OperandSize::Dword)
}

#[test]
fn vpshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 217, 103], OperandSize::Qword)
}

#[test]
fn vpshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 2140371062, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 155, 118, 120, 147, 127, 55], OperandSize::Qword)
}

#[test]
fn vpshufd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 208, 19], OperandSize::Dword)
}

#[test]
fn vpshufd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 12, 192, 49], OperandSize::Dword)
}

#[test]
fn vpshufd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 254, 122], OperandSize::Qword)
}

#[test]
fn vpshufd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 1, 47], OperandSize::Qword)
}

#[test]
fn vpshufd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 112, 216, 70], OperandSize::Dword)
}

#[test]
fn vpshufd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1333494732, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 112, 140, 218, 204, 131, 123, 79, 5], OperandSize::Dword)
}

#[test]
fn vpshufd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 154, 112, 52, 72, 16], OperandSize::Dword)
}

#[test]
fn vpshufd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 125, 141, 112, 214, 84], OperandSize::Qword)
}

#[test]
fn vpshufd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 139, 112, 52, 247, 93], OperandSize::Qword)
}

#[test]
fn vpshufd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 154, 112, 36, 247, 16], OperandSize::Qword)
}

#[test]
fn vpshufd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 112, 233, 29], OperandSize::Dword)
}

#[test]
fn vpshufd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 112, 18, 90], OperandSize::Dword)
}

#[test]
fn vpshufd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 2118354926, Some(OperandSize::Dword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 186, 112, 132, 185, 238, 135, 67, 126, 99], OperandSize::Dword)
}

#[test]
fn vpshufd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 112, 230, 1], OperandSize::Qword)
}

#[test]
fn vpshufd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1685716268, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 112, 44, 125, 44, 253, 121, 100, 83], OperandSize::Qword)
}

#[test]
fn vpshufd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 189, 112, 16, 68], OperandSize::Qword)
}

#[test]
fn vpshufd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 112, 192, 16], OperandSize::Dword)
}

#[test]
fn vpshufd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1708811125, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 112, 4, 69, 117, 99, 218, 101, 94], OperandSize::Dword)
}

#[test]
fn vpshufd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1585710248, Some(OperandSize::Dword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 222, 112, 28, 221, 168, 4, 132, 94, 36], OperandSize::Dword)
}

#[test]
fn vpshufd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 125, 206, 112, 193, 112], OperandSize::Qword)
}

#[test]
fn vpshufd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM28)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 125, 201, 112, 32, 75], OperandSize::Qword)
}

#[test]
fn vpshufd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 125, 218, 112, 12, 73, 10], OperandSize::Qword)
}

