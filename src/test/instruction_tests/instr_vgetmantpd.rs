use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 38, 205, 47], OperandSize::Dword)
}

#[test]
fn vgetmantpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 1532626322, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 38, 128, 146, 5, 90, 91, 92], OperandSize::Dword)
}

#[test]
fn vgetmantpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 158, 38, 36, 216, 59], OperandSize::Dword)
}

#[test]
fn vgetmantpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM17)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 163, 253, 140, 38, 193, 122], OperandSize::Qword)
}

#[test]
fn vgetmantpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1818624528, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 138, 38, 20, 133, 16, 2, 102, 108, 20], OperandSize::Qword)
}

#[test]
fn vgetmantpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(XMM27)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 157, 38, 25, 29], OperandSize::Qword)
}

#[test]
fn vgetmantpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 38, 212, 45], OperandSize::Dword)
}

#[test]
fn vgetmantpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 38, 32, 93], OperandSize::Dword)
}

#[test]
fn vgetmantpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 38, 52, 153, 26], OperandSize::Dword)
}

#[test]
fn vgetmantpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 253, 175, 38, 196, 118], OperandSize::Qword)
}

#[test]
fn vgetmantpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RSI, 152468998, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 253, 174, 38, 150, 6, 126, 22, 9, 82], OperandSize::Qword)
}

#[test]
fn vgetmantpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RBX, 1498267852, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 190, 38, 147, 204, 192, 77, 89, 3], OperandSize::Qword)
}

#[test]
fn vgetmantpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 154, 38, 254, 3], OperandSize::Dword)
}

#[test]
fn vgetmantpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 38, 14, 25], OperandSize::Dword)
}

#[test]
fn vgetmantpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDX, 1407294486, Some(OperandSize::Qword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 38, 178, 22, 156, 225, 83, 87], OperandSize::Dword)
}

#[test]
fn vgetmantpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 38, 224, 123], OperandSize::Qword)
}

#[test]
fn vgetmantpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RAX, 1796092702, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 38, 144, 30, 51, 14, 107, 48], OperandSize::Qword)
}

#[test]
fn vgetmantpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RBX, 1855410568, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 222, 38, 147, 136, 81, 151, 110, 71], OperandSize::Qword)
}

