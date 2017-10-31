use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 105, 233], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1793658387, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 105, 162, 19, 14, 233, 106], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 105, 246], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 996501203, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 105, 188, 115, 211, 102, 101, 59], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 105, 219], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1332687936, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 105, 148, 177, 64, 52, 111, 79], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 105, 236], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RCX, 1383171040, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 105, 145, 224, 131, 113, 82], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 105, 228], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1886496292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 105, 36, 77, 36, 166, 113, 112], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 117, 138, 105, 225], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 61, 139, 105, 60, 177], OperandSize::Qword)
}

