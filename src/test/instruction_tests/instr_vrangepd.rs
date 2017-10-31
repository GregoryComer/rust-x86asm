use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 143, 80, 230, 1], OperandSize::Dword)
}

#[test]
fn vrangepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 397331250, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 143, 80, 36, 69, 50, 203, 174, 23, 57], OperandSize::Dword)
}

#[test]
fn vrangepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 155, 80, 15, 7], OperandSize::Dword)
}

#[test]
fn vrangepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM10)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 237, 138, 80, 242, 96], OperandSize::Qword)
}

#[test]
fn vrangepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 165, 142, 80, 27, 14], OperandSize::Qword)
}

#[test]
fn vrangepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 205, 159, 80, 36, 120, 12], OperandSize::Qword)
}

#[test]
fn vrangepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 237, 171, 80, 248, 119], OperandSize::Dword)
}

#[test]
fn vrangepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1748154885, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 171, 80, 28, 221, 5, 186, 50, 104, 62], OperandSize::Dword)
}

#[test]
fn vrangepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 642497268, Some(OperandSize::Qword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 186, 80, 137, 244, 186, 75, 38, 5], OperandSize::Dword)
}

#[test]
fn vrangepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM17)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 163, 173, 161, 80, 193, 66], OperandSize::Qword)
}

#[test]
fn vrangepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 237, 173, 80, 40, 8], OperandSize::Qword)
}

#[test]
fn vrangepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 657871502, Some(OperandSize::Qword), None)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 213, 183, 80, 188, 119, 142, 82, 54, 39, 23], OperandSize::Qword)
}

#[test]
fn vrangepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 153, 80, 219, 57], OperandSize::Dword)
}

#[test]
fn vrangepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDX, 1509140344, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 202, 80, 154, 120, 167, 243, 89, 85], OperandSize::Dword)
}

#[test]
fn vrangepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 219, 80, 58, 127], OperandSize::Dword)
}

#[test]
fn vrangepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM23)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 163, 213, 157, 80, 199, 99], OperandSize::Qword)
}

#[test]
fn vrangepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 1137529005, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 133, 206, 80, 151, 173, 80, 205, 67, 61], OperandSize::Qword)
}

#[test]
fn vrangepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RBX, 2010849077, Some(OperandSize::Qword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 205, 209, 80, 179, 53, 31, 219, 119, 69], OperandSize::Qword)
}

