use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 63, 243], OperandSize::Dword)
}

#[test]
fn vpmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 564503392, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 63, 12, 149, 96, 163, 165, 33], OperandSize::Dword)
}

#[test]
fn vpmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 63, 248], OperandSize::Qword)
}

#[test]
fn vpmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 309450601, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 63, 186, 105, 215, 113, 18], OperandSize::Qword)
}

#[test]
fn vpmaxud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 63, 218], OperandSize::Dword)
}

#[test]
fn vpmaxud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 63, 12, 242], OperandSize::Dword)
}

#[test]
fn vpmaxud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 63, 217], OperandSize::Qword)
}

#[test]
fn vpmaxud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RAX, 152035943, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 63, 184, 103, 226, 15, 9], OperandSize::Qword)
}

#[test]
fn vpmaxud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 63, 251], OperandSize::Dword)
}

#[test]
fn vpmaxud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 63, 30], OperandSize::Dword)
}

#[test]
fn vpmaxud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 159, 63, 35], OperandSize::Dword)
}

#[test]
fn vpmaxud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 125, 132, 63, 239], OperandSize::Qword)
}

#[test]
fn vpmaxud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1165621889, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 93, 134, 63, 164, 137, 129, 250, 121, 69], OperandSize::Qword)
}

#[test]
fn vpmaxud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RAX, 1425665075, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 149, 63, 160, 51, 236, 249, 84], OperandSize::Qword)
}

#[test]
fn vpmaxud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 63, 252], OperandSize::Dword)
}

#[test]
fn vpmaxud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 63, 52, 195], OperandSize::Dword)
}

#[test]
fn vpmaxud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 63, 52, 74], OperandSize::Dword)
}

#[test]
fn vpmaxud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 117, 171, 63, 237], OperandSize::Qword)
}

#[test]
fn vpmaxud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 594897479, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 101, 175, 63, 4, 213, 71, 106, 117, 35], OperandSize::Qword)
}

#[test]
fn vpmaxud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 170033686, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 109, 188, 63, 140, 139, 22, 130, 34, 10], OperandSize::Qword)
}

#[test]
fn vpmaxud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 207, 63, 242], OperandSize::Dword)
}

#[test]
fn vpmaxud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 63, 17], OperandSize::Dword)
}

#[test]
fn vpmaxud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 63, 2], OperandSize::Dword)
}

#[test]
fn vpmaxud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 117, 194, 63, 212], OperandSize::Qword)
}

#[test]
fn vpmaxud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 2128284613, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 109, 201, 63, 140, 75, 197, 11, 219, 126], OperandSize::Qword)
}

#[test]
fn vpmaxud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 903502702, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 37, 213, 63, 172, 191, 110, 91, 218, 53], OperandSize::Qword)
}

