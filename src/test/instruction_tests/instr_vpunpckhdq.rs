use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 106, 215], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 106, 28, 88], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 106, 249], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1002855076, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 106, 175, 164, 90, 198, 59], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 106, 221], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 358884249, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 106, 135, 153, 35, 100, 21], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 106, 224], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2017344436, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 106, 44, 117, 180, 59, 62, 120], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 106, 198], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 450985588, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 106, 128, 116, 126, 225, 26], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 768992807, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 153, 106, 12, 125, 39, 230, 213, 45], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 101, 129, 106, 232], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 132, 106, 58], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 29, 153, 106, 52, 142], OperandSize::Qword)
}

