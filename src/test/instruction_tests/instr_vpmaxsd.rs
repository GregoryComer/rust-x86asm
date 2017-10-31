use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 61, 236], OperandSize::Dword)
}

#[test]
fn vpmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 61, 20, 158], OperandSize::Dword)
}

#[test]
fn vpmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 61, 204], OperandSize::Qword)
}

#[test]
fn vpmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 61, 23], OperandSize::Qword)
}

#[test]
fn vpmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 61, 193], OperandSize::Dword)
}

#[test]
fn vpmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1584746078, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 61, 36, 253, 94, 78, 117, 94], OperandSize::Dword)
}

#[test]
fn vpmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 61, 230], OperandSize::Qword)
}

#[test]
fn vpmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RSI, 1308797183, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 61, 166, 255, 168, 2, 78], OperandSize::Qword)
}

#[test]
fn vpmaxsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 61, 237], OperandSize::Dword)
}

#[test]
fn vpmaxsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 664675377, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 61, 131, 49, 36, 158, 39], OperandSize::Dword)
}

#[test]
fn vpmaxsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 72457206, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 156, 61, 36, 93, 246, 155, 81, 4], OperandSize::Dword)
}

#[test]
fn vpmaxsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 129, 61, 231], OperandSize::Qword)
}

#[test]
fn vpmaxsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 53, 133, 61, 62], OperandSize::Qword)
}

#[test]
fn vpmaxsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 13, 147, 61, 20, 94], OperandSize::Qword)
}

