use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 221, 140, 80, 198, 97], OperandSize::Dword)
}

#[test]
fn vrangepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 140, 80, 20, 242, 101], OperandSize::Dword)
}

#[test]
fn vrangepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 1083562573, Some(OperandSize::Qword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 156, 80, 182, 77, 218, 149, 64, 8], OperandSize::Dword)
}

#[test]
fn vrangepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM26)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 205, 141, 80, 250, 62], OperandSize::Qword)
}

#[test]
fn vrangepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 229, 134, 80, 4, 248, 9], OperandSize::Qword)
}

#[test]
fn vrangepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 1387430260, Some(OperandSize::Qword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 197, 156, 80, 147, 116, 129, 178, 82, 28], OperandSize::Qword)
}

#[test]
fn vrangepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 175, 80, 247, 57], OperandSize::Dword)
}

#[test]
fn vrangepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 465489090, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 80, 188, 123, 194, 204, 190, 27, 4], OperandSize::Dword)
}

#[test]
fn vrangepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 1040579639, Some(OperandSize::Qword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 205, 187, 80, 185, 55, 252, 5, 62, 53], OperandSize::Dword)
}

#[test]
fn vrangepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 181, 167, 80, 217, 1], OperandSize::Qword)
}

#[test]
fn vrangepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 213, 172, 80, 36, 216, 17], OperandSize::Qword)
}

#[test]
fn vrangepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1138856301, Some(OperandSize::Qword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 245, 185, 80, 140, 123, 109, 145, 225, 67, 2], OperandSize::Qword)
}

#[test]
fn vrangepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 155, 80, 224, 88], OperandSize::Dword)
}

#[test]
fn vrangepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 205, 201, 80, 60, 185, 62], OperandSize::Dword)
}

#[test]
fn vrangepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1447569229, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 219, 80, 4, 133, 77, 39, 72, 86, 76], OperandSize::Dword)
}

#[test]
fn vrangepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM19)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 173, 151, 80, 211, 52], OperandSize::Qword)
}

#[test]
fn vrangepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 165, 203, 80, 8, 59], OperandSize::Qword)
}

#[test]
fn vrangepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 460417061, Some(OperandSize::Qword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 133, 219, 80, 191, 37, 104, 113, 27, 9], OperandSize::Qword)
}

