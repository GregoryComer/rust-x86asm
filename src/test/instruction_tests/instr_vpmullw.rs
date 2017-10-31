use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 213, 212], OperandSize::Dword)
}

#[test]
fn vpmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 370810715, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 213, 184, 91, 31, 26, 22], OperandSize::Dword)
}

#[test]
fn vpmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 213, 222], OperandSize::Qword)
}

#[test]
fn vpmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1523135800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 213, 148, 200, 56, 53, 201, 90], OperandSize::Qword)
}

#[test]
fn vpmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 213, 220], OperandSize::Dword)
}

#[test]
fn vpmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 1907763694, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 213, 136, 238, 41, 182, 113], OperandSize::Dword)
}

#[test]
fn vpmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 213, 242], OperandSize::Qword)
}

#[test]
fn vpmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1567485963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 213, 36, 85, 11, 240, 109, 93], OperandSize::Qword)
}

#[test]
fn vpmullw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 213, 224], OperandSize::Dword)
}

#[test]
fn vpmullw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 838979625, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 213, 12, 77, 41, 208, 1, 50], OperandSize::Dword)
}

#[test]
fn vpmullw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 61, 142, 213, 233], OperandSize::Qword)
}

#[test]
fn vpmullw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 754106623, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 130, 213, 156, 94, 255, 192, 242, 44], OperandSize::Qword)
}

#[test]
fn vpmullw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 213, 239], OperandSize::Dword)
}

#[test]
fn vpmullw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1418038416, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 213, 4, 253, 144, 140, 133, 84], OperandSize::Dword)
}

#[test]
fn vpmullw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 13, 167, 213, 214], OperandSize::Qword)
}

#[test]
fn vpmullw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 45, 166, 213, 44, 192], OperandSize::Qword)
}

#[test]
fn vpmullw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 213, 221], OperandSize::Dword)
}

#[test]
fn vpmullw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 452258637, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 213, 4, 189, 77, 235, 244, 26], OperandSize::Dword)
}

#[test]
fn vpmullw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 5, 198, 213, 195], OperandSize::Qword)
}

#[test]
fn vpmullw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 213, 14], OperandSize::Qword)
}

