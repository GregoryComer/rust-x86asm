use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 202, 22], OperandSize::Dword)
}

#[test]
fn vpshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 20, 118, 60], OperandSize::Dword)
}

#[test]
fn vpshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 223, 30], OperandSize::Qword)
}

#[test]
fn vpshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 49, 98], OperandSize::Qword)
}

#[test]
fn vpshufd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 247, 102], OperandSize::Dword)
}

#[test]
fn vpshufd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 52, 72, 81], OperandSize::Dword)
}

#[test]
fn vpshufd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 204, 65], OperandSize::Qword)
}

#[test]
fn vpshufd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 125627122, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 188, 155, 242, 234, 124, 7, 91], OperandSize::Qword)
}

#[test]
fn vpshufd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 112, 252, 111], OperandSize::Dword)
}

#[test]
fn vpshufd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 112, 60, 208, 125], OperandSize::Dword)
}

#[test]
fn vpshufd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 159, 112, 4, 247, 83], OperandSize::Dword)
}

#[test]
fn vpshufd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 125, 140, 112, 219, 41], OperandSize::Qword)
}

#[test]
fn vpshufd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 112, 28, 193, 64], OperandSize::Qword)
}

#[test]
fn vpshufd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 68278064, Some(OperandSize::Dword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 112, 148, 73, 48, 215, 17, 4, 80], OperandSize::Qword)
}

#[test]
fn vpshufd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 112, 225, 12], OperandSize::Dword)
}

#[test]
fn vpshufd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 2016519413, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 112, 138, 245, 164, 49, 120, 101], OperandSize::Dword)
}

#[test]
fn vpshufd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 189, 112, 63, 71], OperandSize::Dword)
}

#[test]
fn vpshufd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM9)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 125, 174, 112, 249, 15], OperandSize::Qword)
}

#[test]
fn vpshufd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 911807478, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 112, 132, 127, 246, 19, 89, 54, 125], OperandSize::Qword)
}

#[test]
fn vpshufd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 125, 190, 112, 7, 117], OperandSize::Qword)
}

#[test]
fn vpshufd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 112, 195, 89], OperandSize::Dword)
}

#[test]
fn vpshufd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EDI, 2011058620, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 112, 159, 188, 81, 222, 119, 93], OperandSize::Dword)
}

#[test]
fn vpshufd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 220, 112, 60, 247, 106], OperandSize::Dword)
}

#[test]
fn vpshufd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 125, 202, 112, 225, 22], OperandSize::Qword)
}

#[test]
fn vpshufd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 112, 28, 243, 27], OperandSize::Qword)
}

#[test]
fn vpshufd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 217, 112, 38, 45], OperandSize::Qword)
}

