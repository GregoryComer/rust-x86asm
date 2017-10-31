use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 142, 84, 202, 72], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 139, 84, 3, 25], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 157, 84, 40, 1], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 181, 133, 84, 212, 110], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 804058494, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 205, 129, 84, 12, 117, 126, 245, 236, 47, 50], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1014015902, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 205, 149, 84, 156, 185, 158, 167, 112, 60, 87], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 174, 84, 218, 21], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 84, 14, 65], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 632025413, Some(OperandSize::Qword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 191, 84, 52, 93, 69, 241, 171, 37, 10], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 133, 170, 84, 222, 115], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 189, 162, 84, 3, 63], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1393601387, Some(OperandSize::Qword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 187, 84, 44, 205, 107, 171, 16, 83, 52], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 158, 84, 221, 59], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 499705545, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 201, 84, 136, 201, 230, 200, 29, 64], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 1038626289, Some(OperandSize::Qword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 223, 84, 152, 241, 45, 232, 61, 45], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 3, 157, 158, 84, 240, 57], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 237, 195, 84, 62, 93], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 229, 211, 84, 28, 81, 88], OperandSize::Qword)
}

