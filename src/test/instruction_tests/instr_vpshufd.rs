use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 204, 104], OperandSize::Dword)
}

#[test]
fn vpshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 876534072, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 44, 213, 56, 217, 62, 52, 71], OperandSize::Dword)
}

#[test]
fn vpshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 220, 108], OperandSize::Qword)
}

#[test]
fn vpshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1952065104, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 148, 246, 80, 38, 90, 116, 60], OperandSize::Qword)
}

#[test]
fn vpshufd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 223, 116], OperandSize::Dword)
}

#[test]
fn vpshufd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDI, 1762527541, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 143, 53, 9, 14, 105, 19], OperandSize::Dword)
}

#[test]
fn vpshufd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 220, 49], OperandSize::Qword)
}

#[test]
fn vpshufd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RAX, 1188086604, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 184, 76, 195, 208, 70, 73], OperandSize::Qword)
}

#[test]
fn vpshufd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 112, 253, 90], OperandSize::Dword)
}

#[test]
fn vpshufd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 112, 35, 30], OperandSize::Dword)
}

#[test]
fn vpshufd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 492143400, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 112, 129, 40, 131, 85, 29, 22], OperandSize::Dword)
}

#[test]
fn vpshufd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM13)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 125, 138, 112, 245, 36], OperandSize::Qword)
}

#[test]
fn vpshufd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM20)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 142, 112, 32, 97], OperandSize::Qword)
}

#[test]
fn vpshufd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 112, 1, 109], OperandSize::Qword)
}

#[test]
fn vpshufd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 112, 223, 73], OperandSize::Dword)
}

#[test]
fn vpshufd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 112, 4, 214, 69], OperandSize::Dword)
}

#[test]
fn vpshufd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 185, 112, 20, 248, 70], OperandSize::Dword)
}

#[test]
fn vpshufd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM10)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 125, 172, 112, 194, 40], OperandSize::Qword)
}

#[test]
fn vpshufd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 2084455566, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 172, 112, 164, 136, 142, 68, 62, 124, 74], OperandSize::Qword)
}

#[test]
fn vpshufd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1921079287, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 125, 190, 112, 44, 197, 247, 87, 129, 114, 92], OperandSize::Qword)
}

#[test]
fn vpshufd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 112, 237, 86], OperandSize::Dword)
}

#[test]
fn vpshufd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 112, 15, 115], OperandSize::Dword)
}

#[test]
fn vpshufd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 217, 112, 28, 67, 7], OperandSize::Dword)
}

#[test]
fn vpshufd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 202, 112, 235, 8], OperandSize::Qword)
}

#[test]
fn vpshufd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 205, 112, 12, 194, 16], OperandSize::Qword)
}

#[test]
fn vpshufd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 141234950, Some(OperandSize::Dword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 125, 223, 112, 4, 157, 6, 19, 107, 8, 54], OperandSize::Qword)
}

