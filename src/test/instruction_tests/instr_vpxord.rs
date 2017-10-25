use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 239, 225], OperandSize::Dword)
}

#[test]
fn vpxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 239, 60, 217], OperandSize::Dword)
}

#[test]
fn vpxord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 156, 239, 20, 88], OperandSize::Dword)
}

#[test]
fn vpxord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 101, 134, 239, 210], OperandSize::Qword)
}

#[test]
fn vpxord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 142, 239, 63], OperandSize::Qword)
}

#[test]
fn vpxord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1042192627, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 21, 146, 239, 172, 130, 243, 152, 30, 62], OperandSize::Qword)
}

#[test]
fn vpxord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 239, 229], OperandSize::Dword)
}

#[test]
fn vpxord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 239, 41], OperandSize::Dword)
}

#[test]
fn vpxord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1977008208, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 188, 239, 164, 75, 80, 192, 214, 117], OperandSize::Dword)
}

#[test]
fn vpxord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 93, 163, 239, 250], OperandSize::Qword)
}

#[test]
fn vpxord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RAX, 858020858, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 13, 164, 239, 168, 250, 91, 36, 51], OperandSize::Qword)
}

#[test]
fn vpxord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1602973765, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 37, 183, 239, 52, 69, 69, 112, 139, 95], OperandSize::Qword)
}

#[test]
fn vpxord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 239, 254], OperandSize::Dword)
}

#[test]
fn vpxord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 239, 28, 136], OperandSize::Dword)
}

#[test]
fn vpxord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ESI, 641209373, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 220, 239, 150, 29, 20, 56, 38], OperandSize::Dword)
}

#[test]
fn vpxord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 101, 197, 239, 249], OperandSize::Qword)
}

#[test]
fn vpxord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RAX, 1004830034, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 85, 201, 239, 144, 82, 125, 228, 59], OperandSize::Qword)
}

#[test]
fn vpxord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 77454729, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 125, 223, 239, 132, 71, 137, 221, 157, 4], OperandSize::Qword)
}

