use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 231], OperandSize::Dword)
}

#[test]
fn vorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1282626107, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 12, 189, 59, 82, 115, 76], OperandSize::Dword)
}

#[test]
fn vorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 86, 202], OperandSize::Qword)
}

#[test]
fn vorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1712015071, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 131, 223, 70, 11, 102], OperandSize::Qword)
}

#[test]
fn vorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 86, 203], OperandSize::Dword)
}

#[test]
fn vorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 524827288, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 86, 148, 242, 152, 58, 72, 31], OperandSize::Dword)
}

#[test]
fn vorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 86, 209], OperandSize::Qword)
}

#[test]
fn vorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 627735584, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 86, 172, 81, 32, 124, 106, 37], OperandSize::Qword)
}

#[test]
fn vorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 137, 86, 239], OperandSize::Dword)
}

#[test]
fn vorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1752786533, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 86, 12, 149, 101, 102, 121, 104], OperandSize::Dword)
}

#[test]
fn vorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 155, 86, 36, 209], OperandSize::Dword)
}

#[test]
fn vorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 149, 131, 86, 225], OperandSize::Qword)
}

#[test]
fn vorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RCX, 1712914513, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 129, 86, 169, 81, 0, 25, 102], OperandSize::Qword)
}

#[test]
fn vorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RAX, 1946915949, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 189, 148, 86, 160, 109, 148, 11, 116], OperandSize::Qword)
}

#[test]
fn vorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 171, 86, 238], OperandSize::Dword)
}

#[test]
fn vorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 1539006231, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 86, 129, 23, 95, 187, 91], OperandSize::Dword)
}

#[test]
fn vorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 189, 86, 15], OperandSize::Dword)
}

#[test]
fn vorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 141, 162, 86, 221], OperandSize::Qword)
}

#[test]
fn vorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 43941792, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 172, 86, 60, 221, 160, 127, 158, 2], OperandSize::Qword)
}

#[test]
fn vorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 179, 86, 52, 114], OperandSize::Qword)
}

#[test]
fn vorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 201, 86, 242], OperandSize::Dword)
}

#[test]
fn vorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 206, 86, 42], OperandSize::Dword)
}

#[test]
fn vorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 1089763492, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 223, 86, 187, 164, 120, 244, 64], OperandSize::Dword)
}

#[test]
fn vorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 181, 201, 86, 250], OperandSize::Qword)
}

#[test]
fn vorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 173, 207, 86, 52, 211], OperandSize::Qword)
}

#[test]
fn vorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RCX, 70189790, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 237, 214, 86, 169, 222, 2, 47, 4], OperandSize::Qword)
}

