use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 224], OperandSize::Dword)
}

#[test]
fn vpmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1052623565, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 213, 36, 77, 205, 194, 189, 62], OperandSize::Dword)
}

#[test]
fn vpmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 241], OperandSize::Qword)
}

#[test]
fn vpmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 213, 47], OperandSize::Qword)
}

#[test]
fn vpmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 213, 254], OperandSize::Dword)
}

#[test]
fn vpmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1838481274, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 213, 12, 189, 122, 255, 148, 109], OperandSize::Dword)
}

#[test]
fn vpmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 213, 244], OperandSize::Qword)
}

#[test]
fn vpmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 213, 4, 208], OperandSize::Qword)
}

#[test]
fn vpmullw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 213, 228], OperandSize::Dword)
}

#[test]
fn vpmullw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 873275510, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 213, 148, 249, 118, 32, 13, 52], OperandSize::Dword)
}

#[test]
fn vpmullw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 45, 141, 213, 245], OperandSize::Qword)
}

#[test]
fn vpmullw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1943620979, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 141, 213, 148, 71, 115, 77, 217, 115], OperandSize::Qword)
}

#[test]
fn vpmullw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 213, 193], OperandSize::Dword)
}

#[test]
fn vpmullw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 213, 28, 142], OperandSize::Dword)
}

#[test]
fn vpmullw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 101, 172, 213, 208], OperandSize::Qword)
}

#[test]
fn vpmullw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1543060615, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 109, 162, 213, 60, 245, 135, 60, 249, 91], OperandSize::Qword)
}

#[test]
fn vpmullw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 213, 224], OperandSize::Dword)
}

#[test]
fn vpmullw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1863650056, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 213, 148, 79, 8, 11, 21, 111], OperandSize::Dword)
}

#[test]
fn vpmullw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 77, 194, 213, 237], OperandSize::Qword)
}

#[test]
fn vpmullw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 213, 11], OperandSize::Qword)
}

