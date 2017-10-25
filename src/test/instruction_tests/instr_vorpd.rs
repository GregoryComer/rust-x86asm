use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 86, 207], OperandSize::Dword)
}

#[test]
fn vorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1235470302, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 86, 156, 242, 222, 199, 163, 73], OperandSize::Dword)
}

#[test]
fn vorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 86, 239], OperandSize::Qword)
}

#[test]
fn vorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 19], OperandSize::Qword)
}

#[test]
fn vorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 86, 218], OperandSize::Dword)
}

#[test]
fn vorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 878360050, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 86, 190, 242, 181, 90, 52], OperandSize::Dword)
}

#[test]
fn vorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 86, 223], OperandSize::Qword)
}

#[test]
fn vorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 86, 43], OperandSize::Qword)
}

#[test]
fn vorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 86, 215], OperandSize::Dword)
}

#[test]
fn vorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1077957770, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 86, 20, 69, 138, 84, 64, 64], OperandSize::Dword)
}

#[test]
fn vorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1741044929, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 86, 135, 193, 60, 198, 103], OperandSize::Dword)
}

#[test]
fn vorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 245, 139, 86, 239], OperandSize::Qword)
}

#[test]
fn vorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1752858854, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 86, 132, 202, 230, 128, 122, 104], OperandSize::Qword)
}

#[test]
fn vorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 189, 158, 86, 8], OperandSize::Qword)
}

#[test]
fn vorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 170, 86, 254], OperandSize::Dword)
}

#[test]
fn vorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1276980393, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 86, 163, 169, 44, 29, 76], OperandSize::Dword)
}

#[test]
fn vorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 278309387, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 190, 86, 52, 125, 11, 170, 150, 16], OperandSize::Dword)
}

#[test]
fn vorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 205, 164, 86, 196], OperandSize::Qword)
}

#[test]
fn vorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 173, 169, 86, 28, 176], OperandSize::Qword)
}

#[test]
fn vorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1440512259, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 189, 86, 172, 209, 3, 121, 220, 85], OperandSize::Qword)
}

#[test]
fn vorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 205, 86, 233], OperandSize::Dword)
}

#[test]
fn vorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 86, 52, 71], OperandSize::Dword)
}

#[test]
fn vorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 1390405013, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 223, 86, 185, 149, 229, 223, 82], OperandSize::Dword)
}

#[test]
fn vorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 213, 204, 86, 254], OperandSize::Qword)
}

#[test]
fn vorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 195, 86, 52, 200], OperandSize::Qword)
}

#[test]
fn vorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 237, 209, 86, 0], OperandSize::Qword)
}

