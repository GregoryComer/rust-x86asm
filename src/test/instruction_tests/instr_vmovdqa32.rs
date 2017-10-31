use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 111, 201], OperandSize::Dword)
}

#[test]
fn vmovdqa32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1947383226, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 111, 28, 125, 186, 181, 18, 116], OperandSize::Dword)
}

#[test]
fn vmovdqa32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 125, 138, 111, 252], OperandSize::Qword)
}

#[test]
fn vmovdqa32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 141, 111, 44, 246], OperandSize::Qword)
}

#[test]
fn vmovdqa32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqa32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EAX, 112408821, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 184, 245, 56, 179, 6], OperandSize::Dword)
}

#[test]
fn vmovdqa32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 125, 171, 111, 238], OperandSize::Qword)
}

#[test]
fn vmovdqa32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM29)), operand2: Some(IndirectDisplaced(RSI, 1882420948, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 175, 111, 174, 212, 118, 51, 112], OperandSize::Qword)
}

#[test]
fn vmovdqa32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 111, 226], OperandSize::Dword)
}

#[test]
fn vmovdqa32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 111, 6], OperandSize::Dword)
}

#[test]
fn vmovdqa32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 125, 202, 111, 192], OperandSize::Qword)
}

#[test]
fn vmovdqa32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 110283582, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 206, 111, 60, 117, 62, 203, 146, 6], OperandSize::Qword)
}

#[test]
fn vmovdqa32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 235], OperandSize::Dword)
}

#[test]
fn vmovdqa32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 53441845, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 4, 205, 53, 117, 47, 3], OperandSize::Dword)
}

#[test]
fn vmovdqa32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 125, 141, 111, 214], OperandSize::Qword)
}

#[test]
fn vmovdqa32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 8, 127, 25], OperandSize::Qword)
}

#[test]
fn vmovdqa32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 224], OperandSize::Dword)
}

#[test]
fn vmovdqa32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1955334287, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 188, 183, 143, 8, 140, 116], OperandSize::Dword)
}

#[test]
fn vmovdqa32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 125, 173, 111, 194], OperandSize::Qword)
}

#[test]
fn vmovdqa32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1697167504, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 40, 127, 164, 89, 144, 184, 40, 101], OperandSize::Qword)
}

#[test]
fn vmovdqa32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 111, 209], OperandSize::Dword)
}

#[test]
fn vmovdqa32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 52, 151], OperandSize::Dword)
}

#[test]
fn vmovdqa32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 125, 203, 111, 201], OperandSize::Qword)
}

#[test]
fn vmovdqa32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectDisplaced(RBX, 1301399569, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 163, 17, 200, 145, 77], OperandSize::Qword)
}

