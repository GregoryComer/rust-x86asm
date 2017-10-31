use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 220, 205], OperandSize::Dword)
}

#[test]
fn vpaddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1312944605, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 220, 148, 146, 221, 241, 65, 78], OperandSize::Dword)
}

#[test]
fn vpaddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 220, 203], OperandSize::Qword)
}

#[test]
fn vpaddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 220, 44, 202], OperandSize::Qword)
}

#[test]
fn vpaddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 220, 207], OperandSize::Dword)
}

#[test]
fn vpaddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 458979508, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 28, 77, 180, 120, 91, 27], OperandSize::Dword)
}

#[test]
fn vpaddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 238], OperandSize::Qword)
}

#[test]
fn vpaddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 220, 46], OperandSize::Qword)
}

#[test]
fn vpaddusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 220, 228], OperandSize::Dword)
}

#[test]
fn vpaddusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 1567177537, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 220, 142, 65, 59, 105, 93], OperandSize::Dword)
}

#[test]
fn vpaddusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 53, 134, 220, 242], OperandSize::Qword)
}

#[test]
fn vpaddusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 1059148301, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 85, 137, 220, 169, 13, 82, 33, 63], OperandSize::Qword)
}

#[test]
fn vpaddusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 220, 229], OperandSize::Dword)
}

#[test]
fn vpaddusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 220, 8], OperandSize::Dword)
}

#[test]
fn vpaddusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 117, 175, 220, 255], OperandSize::Qword)
}

#[test]
fn vpaddusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 566703943, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 29, 161, 220, 164, 190, 71, 55, 199, 33], OperandSize::Qword)
}

#[test]
fn vpaddusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 220, 234], OperandSize::Dword)
}

#[test]
fn vpaddusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1795643665, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 220, 164, 127, 17, 89, 7, 107], OperandSize::Dword)
}

#[test]
fn vpaddusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 69, 202, 220, 232], OperandSize::Qword)
}

#[test]
fn vpaddusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 13, 205, 220, 60, 113], OperandSize::Qword)
}

