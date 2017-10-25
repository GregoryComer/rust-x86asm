use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 59, 203], OperandSize::Dword)
}

#[test]
fn vpminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 938432790, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 59, 151, 22, 89, 239, 55], OperandSize::Dword)
}

#[test]
fn vpminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 59, 232], OperandSize::Qword)
}

#[test]
fn vpminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 59, 51], OperandSize::Qword)
}

#[test]
fn vpminud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 59, 239], OperandSize::Dword)
}

#[test]
fn vpminud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1535929685, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 59, 36, 245, 85, 109, 140, 91], OperandSize::Dword)
}

#[test]
fn vpminud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 59, 230], OperandSize::Qword)
}

#[test]
fn vpminud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 888720957, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 59, 164, 135, 61, 206, 248, 52], OperandSize::Qword)
}

#[test]
fn vpminud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 59, 192], OperandSize::Dword)
}

#[test]
fn vpminud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 59, 52, 115], OperandSize::Dword)
}

#[test]
fn vpminud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1872899487, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 157, 59, 172, 145, 159, 45, 162, 111], OperandSize::Dword)
}

#[test]
fn vpminud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 13, 143, 59, 215], OperandSize::Qword)
}

#[test]
fn vpminud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2119759891, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 137, 59, 140, 241, 19, 248, 88, 126], OperandSize::Qword)
}

#[test]
fn vpminud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RDX, 918297662, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 5, 149, 59, 178, 62, 28, 188, 54], OperandSize::Qword)
}

#[test]
fn vpminud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 59, 204], OperandSize::Dword)
}

#[test]
fn vpminud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 377312423, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 59, 4, 125, 167, 84, 125, 22], OperandSize::Dword)
}

#[test]
fn vpminud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 848773511, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 185, 59, 44, 157, 135, 65, 151, 50], OperandSize::Dword)
}

#[test]
fn vpminud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 117, 161, 59, 228], OperandSize::Qword)
}

#[test]
fn vpminud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 1836802717, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 77, 175, 59, 178, 157, 98, 123, 109], OperandSize::Qword)
}

#[test]
fn vpminud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RAX, 1348938611, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 117, 179, 59, 152, 115, 43, 103, 80], OperandSize::Qword)
}

#[test]
fn vpminud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 59, 247], OperandSize::Dword)
}

#[test]
fn vpminud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1899699676, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 59, 160, 220, 29, 59, 113], OperandSize::Dword)
}

#[test]
fn vpminud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 221, 59, 6], OperandSize::Dword)
}

#[test]
fn vpminud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 85, 202, 59, 226], OperandSize::Qword)
}

#[test]
fn vpminud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RDX, 1836715258, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 45, 198, 59, 138, 250, 12, 122, 109], OperandSize::Qword)
}

#[test]
fn vpminud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1883343575, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 61, 219, 59, 28, 77, 215, 138, 65, 112], OperandSize::Qword)
}

