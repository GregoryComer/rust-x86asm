use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 246, 64], OperandSize::Dword)
}

#[test]
fn vpshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1546674085, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 4, 85, 165, 95, 48, 92, 74], OperandSize::Dword)
}

#[test]
fn vpshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 205, 110], OperandSize::Qword)
}

#[test]
fn vpshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 657825800, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 112, 161, 8, 160, 53, 39, 3], OperandSize::Qword)
}

#[test]
fn vpshufd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 208, 64], OperandSize::Dword)
}

#[test]
fn vpshufd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 43, 50], OperandSize::Dword)
}

#[test]
fn vpshufd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 210, 62], OperandSize::Qword)
}

#[test]
fn vpshufd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RAX, 625087015, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 112, 168, 39, 18, 66, 37, 50], OperandSize::Qword)
}

#[test]
fn vpshufd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 112, 252, 102], OperandSize::Dword)
}

#[test]
fn vpshufd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 786757887, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 112, 20, 69, 255, 248, 228, 46, 40], OperandSize::Dword)
}

#[test]
fn vpshufd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 153, 112, 23, 84], OperandSize::Dword)
}

#[test]
fn vpshufd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 125, 141, 112, 220, 116], OperandSize::Qword)
}

#[test]
fn vpshufd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 635679579, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 112, 174, 91, 179, 227, 37, 36], OperandSize::Qword)
}

#[test]
fn vpshufd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1106157815, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 125, 157, 112, 148, 218, 247, 160, 238, 65, 22], OperandSize::Qword)
}

#[test]
fn vpshufd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 112, 237, 36], OperandSize::Dword)
}

#[test]
fn vpshufd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 500606149, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 112, 148, 178, 197, 164, 214, 29, 111], OperandSize::Dword)
}

#[test]
fn vpshufd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EBX, 1590024122, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 187, 112, 131, 186, 215, 197, 94, 117], OperandSize::Dword)
}

#[test]
fn vpshufd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 125, 172, 112, 220, 117], OperandSize::Qword)
}

#[test]
fn vpshufd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 343626096, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 125, 175, 112, 180, 154, 112, 81, 123, 20, 119], OperandSize::Qword)
}

#[test]
fn vpshufd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 125, 191, 112, 20, 89, 55], OperandSize::Qword)
}

#[test]
fn vpshufd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 112, 194, 104], OperandSize::Dword)
}

#[test]
fn vpshufd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 112, 2, 31], OperandSize::Dword)
}

#[test]
fn vpshufd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ESI, 106071097, Some(OperandSize::Dword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 222, 112, 174, 57, 132, 82, 6, 38], OperandSize::Dword)
}

#[test]
fn vpshufd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 202, 112, 242, 8], OperandSize::Qword)
}

#[test]
fn vpshufd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1147648102, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 207, 112, 156, 243, 102, 184, 103, 68, 36], OperandSize::Qword)
}

#[test]
fn vpshufd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFD, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 125, 220, 112, 44, 207, 1], OperandSize::Qword)
}

