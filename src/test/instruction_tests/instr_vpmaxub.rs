use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 222, 248], OperandSize::Dword)
}

#[test]
fn vpmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1570707046, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 222, 150, 102, 22, 159, 93], OperandSize::Dword)
}

#[test]
fn vpmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 222, 235], OperandSize::Qword)
}

#[test]
fn vpmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 222, 23], OperandSize::Qword)
}

#[test]
fn vpmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 222, 220], OperandSize::Dword)
}

#[test]
fn vpmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 492159963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 222, 12, 149, 219, 195, 85, 29], OperandSize::Dword)
}

#[test]
fn vpmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 222, 229], OperandSize::Qword)
}

#[test]
fn vpmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 222, 52, 146], OperandSize::Qword)
}

#[test]
fn vpmaxub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 222, 253], OperandSize::Dword)
}

#[test]
fn vpmaxub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 315240509, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 222, 166, 61, 48, 202, 18], OperandSize::Dword)
}

#[test]
fn vpmaxub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 101, 131, 222, 237], OperandSize::Qword)
}

#[test]
fn vpmaxub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 1583776613, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 109, 140, 222, 138, 101, 131, 102, 94], OperandSize::Qword)
}

#[test]
fn vpmaxub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 222, 233], OperandSize::Dword)
}

#[test]
fn vpmaxub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 172, 222, 44, 94], OperandSize::Dword)
}

#[test]
fn vpmaxub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 61, 161, 222, 254], OperandSize::Qword)
}

#[test]
fn vpmaxub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 69, 171, 222, 20, 191], OperandSize::Qword)
}

#[test]
fn vpmaxub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 222, 197], OperandSize::Dword)
}

#[test]
fn vpmaxub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 723424837, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 201, 222, 12, 149, 69, 150, 30, 43], OperandSize::Dword)
}

#[test]
fn vpmaxub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 193, 222, 232], OperandSize::Qword)
}

#[test]
fn vpmaxub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RAX, 1099310422, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 85, 201, 222, 128, 86, 37, 134, 65], OperandSize::Qword)
}

