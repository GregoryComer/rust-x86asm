use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 20, 200], OperandSize::Dword)
}

#[test]
fn vunpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 341624745, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 20, 186, 169, 199, 92, 20], OperandSize::Dword)
}

#[test]
fn vunpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 20, 241], OperandSize::Qword)
}

#[test]
fn vunpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDX, 246272085, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 20, 138, 85, 208, 173, 14], OperandSize::Qword)
}

#[test]
fn vunpcklpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 20, 229], OperandSize::Dword)
}

#[test]
fn vunpcklpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 20, 20, 74], OperandSize::Dword)
}

#[test]
fn vunpcklpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 20, 234], OperandSize::Qword)
}

#[test]
fn vunpcklpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 441785287, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 20, 172, 159, 199, 27, 85, 26], OperandSize::Qword)
}

#[test]
fn vunpcklpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 20, 252], OperandSize::Dword)
}

#[test]
fn vunpcklpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1569032720, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 20, 154, 16, 138, 133, 93], OperandSize::Dword)
}

#[test]
fn vunpcklpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 20, 12, 248], OperandSize::Dword)
}

#[test]
fn vunpcklpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 149, 129, 20, 194], OperandSize::Qword)
}

#[test]
fn vunpcklpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 649713224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 245, 140, 20, 176, 72, 214, 185, 38], OperandSize::Qword)
}

#[test]
fn vunpcklpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 181, 155, 20, 59], OperandSize::Qword)
}

#[test]
fn vunpcklpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 20, 236], OperandSize::Dword)
}

#[test]
fn vunpcklpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 325307007, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 20, 28, 93, 127, 202, 99, 19], OperandSize::Dword)
}

#[test]
fn vunpcklpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 20, 30], OperandSize::Dword)
}

#[test]
fn vunpcklpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 245, 166, 20, 247], OperandSize::Qword)
}

#[test]
fn vunpcklpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 157, 174, 20, 4, 131], OperandSize::Qword)
}

#[test]
fn vunpcklpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1533147891, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 157, 185, 20, 60, 189, 243, 250, 97, 91], OperandSize::Qword)
}

#[test]
fn vunpcklpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 203, 20, 217], OperandSize::Dword)
}

#[test]
fn vunpcklpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 20, 59], OperandSize::Dword)
}

#[test]
fn vunpcklpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 221, 20, 52, 67], OperandSize::Dword)
}

#[test]
fn vunpcklpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 149, 194, 20, 197], OperandSize::Qword)
}

#[test]
fn vunpcklpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 244162625, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 20, 28, 189, 65, 160, 141, 14], OperandSize::Qword)
}

#[test]
fn vunpcklpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RSI, 189626101, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 253, 218, 20, 150, 245, 118, 77, 11], OperandSize::Qword)
}

