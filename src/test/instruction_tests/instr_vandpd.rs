use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 249], OperandSize::Dword)
}

#[test]
fn vandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 84, 11], OperandSize::Dword)
}

#[test]
fn vandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 84, 211], OperandSize::Qword)
}

#[test]
fn vandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 627250086, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 84, 4, 157, 166, 19, 99, 37], OperandSize::Qword)
}

#[test]
fn vandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 84, 211], OperandSize::Dword)
}

#[test]
fn vandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 84, 28, 183], OperandSize::Dword)
}

#[test]
fn vandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 84, 227], OperandSize::Qword)
}

#[test]
fn vandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 396240273, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 84, 4, 85, 145, 37, 158, 23], OperandSize::Qword)
}

#[test]
fn vandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 141, 84, 199], OperandSize::Dword)
}

#[test]
fn vandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 84, 46], OperandSize::Dword)
}

#[test]
fn vandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 2102641631, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 153, 84, 188, 223, 223, 195, 83, 125], OperandSize::Dword)
}

#[test]
fn vandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 181, 129, 84, 253], OperandSize::Qword)
}

#[test]
fn vandpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 149, 132, 84, 28, 208], OperandSize::Qword)
}

#[test]
fn vandpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RSI, 1557400833, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 181, 156, 84, 190, 1, 13, 212, 92], OperandSize::Qword)
}

#[test]
fn vandpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 84, 241], OperandSize::Dword)
}

#[test]
fn vandpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 881919989, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 84, 44, 149, 245, 7, 145, 52], OperandSize::Dword)
}

#[test]
fn vandpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 190, 84, 17], OperandSize::Dword)
}

#[test]
fn vandpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 197, 174, 84, 235], OperandSize::Qword)
}

#[test]
fn vandpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1055701153, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 141, 170, 84, 148, 217, 161, 184, 236, 62], OperandSize::Qword)
}

#[test]
fn vandpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RCX, 954269222, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 182, 84, 145, 38, 254, 224, 56], OperandSize::Qword)
}

#[test]
fn vandpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 206, 84, 255], OperandSize::Dword)
}

#[test]
fn vandpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 84, 52, 146], OperandSize::Dword)
}

#[test]
fn vandpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 144512996, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 222, 84, 44, 69, 228, 23, 157, 8], OperandSize::Dword)
}

#[test]
fn vandpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 229, 195, 84, 251], OperandSize::Qword)
}

#[test]
fn vandpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1873037920, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 213, 207, 84, 164, 192, 96, 74, 164, 111], OperandSize::Qword)
}

#[test]
fn vandpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RDX, 780106369, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 213, 84, 170, 129, 122, 127, 46], OperandSize::Qword)
}

