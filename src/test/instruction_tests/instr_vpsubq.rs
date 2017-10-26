use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 251, 247], OperandSize::Dword)
}

#[test]
fn vpsubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1275696237, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 251, 144, 109, 148, 9, 76], OperandSize::Dword)
}

#[test]
fn vpsubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 251, 232], OperandSize::Qword)
}

#[test]
fn vpsubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 847504868, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 251, 28, 213, 228, 229, 131, 50], OperandSize::Qword)
}

#[test]
fn vpsubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 220], OperandSize::Dword)
}

#[test]
fn vpsubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 12, 66], OperandSize::Dword)
}

#[test]
fn vpsubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 218], OperandSize::Qword)
}

#[test]
fn vpsubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 1130072729, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 251, 169, 153, 138, 91, 67], OperandSize::Qword)
}

#[test]
fn vpsubq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 251, 246], OperandSize::Dword)
}

#[test]
fn vpsubq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 139, 251, 52, 185], OperandSize::Dword)
}

#[test]
fn vpsubq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 503070290, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 159, 251, 28, 197, 82, 62, 252, 29], OperandSize::Dword)
}

#[test]
fn vpsubq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 133, 133, 251, 250], OperandSize::Qword)
}

#[test]
fn vpsubq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 229, 143, 251, 20, 200], OperandSize::Qword)
}

#[test]
fn vpsubq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RSI, 1342561340, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 245, 149, 251, 134, 60, 220, 5, 80], OperandSize::Qword)
}

#[test]
fn vpsubq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 173, 251, 199], OperandSize::Dword)
}

#[test]
fn vpsubq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 1341100438, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 251, 138, 150, 145, 239, 79], OperandSize::Dword)
}

#[test]
fn vpsubq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 284223498, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 189, 251, 164, 123, 10, 232, 240, 16], OperandSize::Dword)
}

#[test]
fn vpsubq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 173, 171, 251, 193], OperandSize::Qword)
}

#[test]
fn vpsubq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 237191405, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 181, 161, 251, 4, 149, 237, 64, 35, 14], OperandSize::Qword)
}

#[test]
fn vpsubq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 189, 251, 12, 79], OperandSize::Qword)
}

#[test]
fn vpsubq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 251, 209], OperandSize::Dword)
}

#[test]
fn vpsubq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 251, 40], OperandSize::Dword)
}

#[test]
fn vpsubq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 1127294460, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 223, 251, 152, 252, 37, 49, 67], OperandSize::Dword)
}

#[test]
fn vpsubq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 245, 207, 251, 246], OperandSize::Qword)
}

#[test]
fn vpsubq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 278489906, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 245, 207, 251, 140, 254, 50, 107, 153, 16], OperandSize::Qword)
}

#[test]
fn vpsubq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 253, 211, 251, 30], OperandSize::Qword)
}

