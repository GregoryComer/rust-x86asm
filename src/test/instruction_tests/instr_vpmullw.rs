use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 213, 227], OperandSize::Dword)
}

#[test]
fn vpmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 213, 10], OperandSize::Dword)
}

#[test]
fn vpmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 213, 244], OperandSize::Qword)
}

#[test]
fn vpmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 1227979365, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 182, 101, 122, 49, 73], OperandSize::Qword)
}

#[test]
fn vpmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 213, 226], OperandSize::Dword)
}

#[test]
fn vpmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2040508872, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 213, 60, 205, 200, 177, 159, 121], OperandSize::Dword)
}

#[test]
fn vpmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 213, 254], OperandSize::Qword)
}

#[test]
fn vpmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2107990650, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 213, 36, 125, 122, 98, 165, 125], OperandSize::Qword)
}

#[test]
fn vpmullw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 140, 213, 199], OperandSize::Dword)
}

#[test]
fn vpmullw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 213, 55], OperandSize::Dword)
}

#[test]
fn vpmullw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 109, 131, 213, 194], OperandSize::Qword)
}

#[test]
fn vpmullw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 29, 139, 213, 44, 72], OperandSize::Qword)
}

#[test]
fn vpmullw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 213, 219], OperandSize::Dword)
}

#[test]
fn vpmullw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 213, 57], OperandSize::Dword)
}

#[test]
fn vpmullw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 53, 165, 213, 239], OperandSize::Qword)
}

#[test]
fn vpmullw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1130319559, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 170, 213, 188, 70, 199, 78, 95, 67], OperandSize::Qword)
}

#[test]
fn vpmullw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 213, 219], OperandSize::Dword)
}

#[test]
fn vpmullw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1644430316, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 204, 213, 4, 133, 236, 3, 4, 98], OperandSize::Dword)
}

#[test]
fn vpmullw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 203, 213, 236], OperandSize::Qword)
}

#[test]
fn vpmullw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 309407322, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 203, 213, 60, 133, 90, 46, 113, 18], OperandSize::Qword)
}

