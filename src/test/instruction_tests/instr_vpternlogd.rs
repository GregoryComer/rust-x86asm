use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 37, 200, 36], OperandSize::Dword)
}

#[test]
fn vpternlogd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 873754812, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 140, 37, 140, 187, 188, 112, 20, 52, 47], OperandSize::Dword)
}

#[test]
fn vpternlogd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 158, 37, 30, 76], OperandSize::Dword)
}

#[test]
fn vpternlogd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 137, 37, 196, 58], OperandSize::Qword)
}

#[test]
fn vpternlogd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1008888565, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 61, 137, 37, 60, 197, 245, 106, 34, 60, 59], OperandSize::Qword)
}

#[test]
fn vpternlogd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1969740716, Some(OperandSize::Dword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 45, 148, 37, 180, 151, 172, 219, 103, 117, 86], OperandSize::Qword)
}

#[test]
fn vpternlogd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 171, 37, 213, 115], OperandSize::Dword)
}

#[test]
fn vpternlogd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 117, 171, 37, 27, 14], OperandSize::Dword)
}

#[test]
fn vpternlogd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 77, 186, 37, 57, 48], OperandSize::Dword)
}

#[test]
fn vpternlogd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 69, 174, 37, 217, 9], OperandSize::Qword)
}

#[test]
fn vpternlogd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 125, 167, 37, 32, 28], OperandSize::Qword)
}

#[test]
fn vpternlogd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 77, 177, 37, 36, 78, 73], OperandSize::Qword)
}

#[test]
fn vpternlogd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 201, 37, 208, 74], OperandSize::Dword)
}

#[test]
fn vpternlogd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 204, 37, 36, 71, 31], OperandSize::Dword)
}

#[test]
fn vpternlogd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 44645270, Some(OperandSize::Dword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 223, 37, 20, 253, 150, 59, 169, 2, 87], OperandSize::Dword)
}

#[test]
fn vpternlogd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 5, 199, 37, 210, 66], OperandSize::Qword)
}

#[test]
fn vpternlogd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1190549837, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 77, 204, 37, 20, 133, 77, 89, 246, 70, 59], OperandSize::Qword)
}

#[test]
fn vpternlogd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 45, 223, 37, 63, 54], OperandSize::Qword)
}

