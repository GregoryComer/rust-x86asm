use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 251, 229], OperandSize::Dword)
}

#[test]
fn vpsubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 96167898, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 251, 44, 149, 218, 103, 187, 5], OperandSize::Dword)
}

#[test]
fn vpsubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 251, 246], OperandSize::Qword)
}

#[test]
fn vpsubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 82517434, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 251, 20, 133, 186, 29, 235, 4], OperandSize::Qword)
}

#[test]
fn vpsubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 239], OperandSize::Dword)
}

#[test]
fn vpsubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1281592752, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 251, 179, 176, 141, 99, 76], OperandSize::Dword)
}

#[test]
fn vpsubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 251, 232], OperandSize::Qword)
}

#[test]
fn vpsubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1952290415, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 251, 20, 149, 111, 150, 93, 116], OperandSize::Qword)
}

#[test]
fn vpsubq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 138, 251, 237], OperandSize::Dword)
}

#[test]
fn vpsubq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 137, 251, 15], OperandSize::Dword)
}

#[test]
fn vpsubq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1174208613, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 156, 251, 52, 181, 101, 0, 253, 69], OperandSize::Dword)
}

#[test]
fn vpsubq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 173, 134, 251, 247], OperandSize::Qword)
}

#[test]
fn vpsubq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 189, 139, 251, 0], OperandSize::Qword)
}

#[test]
fn vpsubq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1250333871, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 157, 155, 251, 4, 77, 175, 148, 134, 74], OperandSize::Qword)
}

#[test]
fn vpsubq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 251, 250], OperandSize::Dword)
}

#[test]
fn vpsubq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 251, 41], OperandSize::Dword)
}

#[test]
fn vpsubq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1131390514, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 187, 251, 138, 50, 166, 111, 67], OperandSize::Dword)
}

#[test]
fn vpsubq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 157, 162, 251, 230], OperandSize::Qword)
}

#[test]
fn vpsubq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1251149643, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 181, 173, 251, 156, 250, 75, 7, 147, 74], OperandSize::Qword)
}

#[test]
fn vpsubq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 187, 251, 35], OperandSize::Qword)
}

#[test]
fn vpsubq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 204, 251, 193], OperandSize::Dword)
}

#[test]
fn vpsubq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1857688158, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 251, 132, 255, 94, 18, 186, 110], OperandSize::Dword)
}

#[test]
fn vpsubq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 218, 251, 60, 246], OperandSize::Dword)
}

#[test]
fn vpsubq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 149, 193, 251, 209], OperandSize::Qword)
}

#[test]
fn vpsubq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 173, 202, 251, 15], OperandSize::Qword)
}

#[test]
fn vpsubq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 133, 210, 251, 56], OperandSize::Qword)
}

