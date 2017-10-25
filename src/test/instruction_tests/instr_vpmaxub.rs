use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 222, 248], OperandSize::Dword)
}

#[test]
fn vpmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 71544251, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 222, 12, 93, 187, 173, 67, 4], OperandSize::Dword)
}

#[test]
fn vpmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 222, 200], OperandSize::Qword)
}

#[test]
fn vpmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 820049935, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 222, 28, 77, 15, 248, 224, 48], OperandSize::Qword)
}

#[test]
fn vpmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 222, 239], OperandSize::Dword)
}

#[test]
fn vpmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 222, 14], OperandSize::Dword)
}

#[test]
fn vpmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 222, 217], OperandSize::Qword)
}

#[test]
fn vpmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 222, 33], OperandSize::Qword)
}

#[test]
fn vpmaxub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 222, 241], OperandSize::Dword)
}

#[test]
fn vpmaxub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1000822711, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 222, 12, 189, 183, 87, 167, 59], OperandSize::Dword)
}

#[test]
fn vpmaxub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 37, 137, 222, 225], OperandSize::Qword)
}

#[test]
fn vpmaxub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 109, 130, 222, 52, 254], OperandSize::Qword)
}

#[test]
fn vpmaxub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 222, 246], OperandSize::Dword)
}

#[test]
fn vpmaxub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 985713408, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 222, 12, 205, 0, 203, 192, 58], OperandSize::Dword)
}

#[test]
fn vpmaxub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 93, 164, 222, 230], OperandSize::Qword)
}

#[test]
fn vpmaxub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 69, 174, 222, 41], OperandSize::Qword)
}

#[test]
fn vpmaxub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 222, 241], OperandSize::Dword)
}

#[test]
fn vpmaxub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 222, 14], OperandSize::Dword)
}

#[test]
fn vpmaxub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 53, 201, 222, 245], OperandSize::Qword)
}

#[test]
fn vpmaxub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 196, 222, 20, 81], OperandSize::Qword)
}

