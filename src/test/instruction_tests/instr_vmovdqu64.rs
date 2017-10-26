use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 111, 224], OperandSize::Dword)
}

#[test]
fn vmovdqu64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 111, 44, 216], OperandSize::Dword)
}

#[test]
fn vmovdqu64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 254, 142, 111, 192], OperandSize::Qword)
}

#[test]
fn vmovdqu64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1445857609, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 254, 139, 111, 52, 77, 73, 9, 46, 86], OperandSize::Qword)
}

#[test]
fn vmovdqu64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 111, 200], OperandSize::Dword)
}

#[test]
fn vmovdqu64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 111, 63], OperandSize::Dword)
}

#[test]
fn vmovdqu64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 254, 169, 111, 222], OperandSize::Qword)
}

#[test]
fn vmovdqu64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM30)), operand2: Some(IndirectDisplaced(RSI, 304493424, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 254, 173, 111, 182, 112, 51, 38, 18], OperandSize::Qword)
}

#[test]
fn vmovdqu64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 203, 111, 246], OperandSize::Dword)
}

#[test]
fn vmovdqu64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 207, 111, 12, 88], OperandSize::Dword)
}

#[test]
fn vmovdqu64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 254, 205, 111, 235], OperandSize::Qword)
}

#[test]
fn vmovdqu64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 204, 111, 35], OperandSize::Qword)
}

#[test]
fn vmovdqu64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 111, 251], OperandSize::Dword)
}

#[test]
fn vmovdqu64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectDisplaced(ECX, 1935919517, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 145, 157, 201, 99, 115], OperandSize::Dword)
}

#[test]
fn vmovdqu64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 254, 141, 111, 201], OperandSize::Qword)
}

#[test]
fn vmovdqu64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 254, 8, 127, 19], OperandSize::Qword)
}

#[test]
fn vmovdqu64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 111, 252], OperandSize::Dword)
}

#[test]
fn vmovdqu64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 20, 73], OperandSize::Dword)
}

#[test]
fn vmovdqu64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 254, 170, 111, 251], OperandSize::Qword)
}

#[test]
fn vmovdqu64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectDisplaced(RDI, 952637128, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 254, 40, 127, 159, 200, 22, 200, 56], OperandSize::Qword)
}

#[test]
fn vmovdqu64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 202, 111, 205], OperandSize::Dword)
}

#[test]
fn vmovdqu64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 12, 144], OperandSize::Dword)
}

#[test]
fn vmovdqu64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 254, 205, 111, 236], OperandSize::Qword)
}

#[test]
fn vmovdqu64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 254, 72, 127, 48], OperandSize::Qword)
}

