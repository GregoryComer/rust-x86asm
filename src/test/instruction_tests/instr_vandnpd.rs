use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 85, 222], OperandSize::Dword)
}

#[test]
fn vandnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 925484945, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 85, 60, 157, 145, 199, 41, 55], OperandSize::Dword)
}

#[test]
fn vandnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 85, 237], OperandSize::Qword)
}

#[test]
fn vandnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 85, 34], OperandSize::Qword)
}

#[test]
fn vandnpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 85, 226], OperandSize::Dword)
}

#[test]
fn vandnpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 85, 20, 151], OperandSize::Dword)
}

#[test]
fn vandnpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 85, 210], OperandSize::Qword)
}

#[test]
fn vandnpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 204790480, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 85, 161, 208, 218, 52, 12], OperandSize::Qword)
}

#[test]
fn vandnpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 143, 85, 231], OperandSize::Dword)
}

#[test]
fn vandnpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1898276322, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 138, 85, 128, 226, 101, 37, 113], OperandSize::Dword)
}

#[test]
fn vandnpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 395992628, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 156, 85, 166, 52, 94, 154, 23], OperandSize::Dword)
}

#[test]
fn vandnpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 197, 143, 85, 234], OperandSize::Qword)
}

#[test]
fn vandnpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 205, 143, 85, 31], OperandSize::Qword)
}

#[test]
fn vandnpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 2088157267, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 133, 148, 85, 177, 83, 192, 118, 124], OperandSize::Qword)
}

#[test]
fn vandnpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 85, 233], OperandSize::Dword)
}

#[test]
fn vandnpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 426378311, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 85, 140, 138, 71, 4, 106, 25], OperandSize::Dword)
}

#[test]
fn vandnpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 403133368, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 189, 85, 176, 184, 83, 7, 24], OperandSize::Dword)
}

#[test]
fn vandnpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 133, 162, 85, 229], OperandSize::Qword)
}

#[test]
fn vandnpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 676421526, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 157, 163, 85, 188, 94, 150, 95, 81, 40], OperandSize::Qword)
}

#[test]
fn vandnpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 229, 188, 85, 16], OperandSize::Qword)
}

#[test]
fn vandnpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 85, 205], OperandSize::Dword)
}

#[test]
fn vandnpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 204, 85, 26], OperandSize::Dword)
}

#[test]
fn vandnpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 1344812559, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 218, 85, 137, 15, 54, 40, 80], OperandSize::Dword)
}

#[test]
fn vandnpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 197, 85, 227], OperandSize::Qword)
}

#[test]
fn vandnpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 221, 197, 85, 52, 255], OperandSize::Qword)
}

#[test]
fn vandnpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 205, 213, 85, 14], OperandSize::Qword)
}

