use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 19, 222], OperandSize::Dword)
}

#[test]
fn vpmovusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1746227089, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 164, 183, 145, 79, 21, 104], OperandSize::Dword)
}

#[test]
fn vpmovusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 126, 143, 19, 216], OperandSize::Qword)
}

#[test]
fn vpmovusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(RSI, 185223655, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 19, 158, 231, 73, 10, 11], OperandSize::Qword)
}

#[test]
fn vpmovusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 19, 250], OperandSize::Dword)
}

#[test]
fn vpmovusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1530063681, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 172, 65, 65, 235, 50, 91], OperandSize::Dword)
}

#[test]
fn vpmovusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 19, 202], OperandSize::Qword)
}

#[test]
fn vpmovusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RSI, Two, 2038825611, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 19, 28, 117, 139, 2, 134, 121], OperandSize::Qword)
}

#[test]
fn vpmovusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 19, 255], OperandSize::Dword)
}

#[test]
fn vpmovusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(ESI, 737793135, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 150, 111, 212, 249, 43], OperandSize::Dword)
}

#[test]
fn vpmovusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 126, 201, 19, 196], OperandSize::Qword)
}

#[test]
fn vpmovusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 19, 30], OperandSize::Qword)
}

