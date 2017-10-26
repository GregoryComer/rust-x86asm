use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 21, 199], OperandSize::Dword)
}

#[test]
fn vunpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 131295757, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 21, 130, 13, 106, 211, 7], OperandSize::Dword)
}

#[test]
fn vunpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 21, 215], OperandSize::Qword)
}

#[test]
fn vunpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 332987269, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 21, 172, 207, 133, 251, 216, 19], OperandSize::Qword)
}

#[test]
fn vunpckhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 21, 200], OperandSize::Dword)
}

#[test]
fn vunpckhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1772993714, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 21, 36, 149, 178, 188, 173, 105], OperandSize::Dword)
}

#[test]
fn vunpckhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 21, 201], OperandSize::Qword)
}

#[test]
fn vunpckhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 189723305, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 21, 20, 125, 169, 242, 78, 11], OperandSize::Qword)
}

#[test]
fn vunpckhpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 21, 251], OperandSize::Dword)
}

#[test]
fn vunpckhpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 139, 21, 36, 87], OperandSize::Dword)
}

#[test]
fn vunpckhpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1291590837, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 156, 21, 148, 122, 181, 28, 252, 76], OperandSize::Dword)
}

#[test]
fn vunpckhpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 165, 139, 21, 232], OperandSize::Qword)
}

#[test]
fn vunpckhpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RAX, 1023903925, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 213, 140, 21, 160, 181, 136, 7, 61], OperandSize::Qword)
}

#[test]
fn vunpckhpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 581695473, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 229, 148, 21, 172, 214, 241, 247, 171, 34], OperandSize::Qword)
}

#[test]
fn vunpckhpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 21, 241], OperandSize::Dword)
}

#[test]
fn vunpckhpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 2030066819, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 169, 21, 140, 187, 131, 92, 0, 121], OperandSize::Dword)
}

#[test]
fn vunpckhpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 436150825, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 190, 21, 44, 181, 41, 34, 255, 25], OperandSize::Dword)
}

#[test]
fn vunpckhpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 165, 161, 21, 193], OperandSize::Qword)
}

#[test]
fn vunpckhpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 819901316, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 229, 175, 21, 148, 144, 132, 179, 222, 48], OperandSize::Qword)
}

#[test]
fn vunpckhpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1156525493, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 181, 190, 21, 148, 183, 181, 45, 239, 68], OperandSize::Qword)
}

#[test]
fn vunpckhpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 205, 21, 196], OperandSize::Dword)
}

#[test]
fn vunpckhpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 21, 33], OperandSize::Dword)
}

#[test]
fn vunpckhpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1929582036, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 223, 21, 132, 112, 212, 21, 3, 115], OperandSize::Dword)
}

#[test]
fn vunpckhpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 173, 203, 21, 193], OperandSize::Qword)
}

#[test]
fn vunpckhpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 237, 201, 21, 49], OperandSize::Qword)
}

#[test]
fn vunpckhpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 209, 21, 26], OperandSize::Qword)
}

