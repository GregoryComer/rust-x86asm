use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 139, 210], OperandSize::Dword)
}

#[test]
fn vpcompressq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 717439868, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 139, 140, 87, 124, 67, 195, 42], OperandSize::Dword)
}

#[test]
fn vpcompressq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 253, 140, 139, 254], OperandSize::Qword)
}

#[test]
fn vpcompressq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectDisplaced(RBX, 1216370670, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 8, 139, 171, 238, 87, 128, 72], OperandSize::Qword)
}

#[test]
fn vpcompressq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 139, 246], OperandSize::Dword)
}

#[test]
fn vpcompressq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 139, 12, 90], OperandSize::Dword)
}

#[test]
fn vpcompressq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 253, 174, 139, 214], OperandSize::Qword)
}

#[test]
fn vpcompressq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1010909253, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 40, 139, 148, 146, 69, 64, 65, 60], OperandSize::Qword)
}

#[test]
fn vpcompressq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 139, 252], OperandSize::Dword)
}

#[test]
fn vpcompressq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 44, 134], OperandSize::Dword)
}

#[test]
fn vpcompressq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 253, 205, 139, 196], OperandSize::Qword)
}

#[test]
fn vpcompressq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 52, 142], OperandSize::Qword)
}

