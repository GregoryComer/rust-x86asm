use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1739367387, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 132, 146, 219, 163, 172, 103], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 28, 123], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 247], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 226], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 25, 223], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 25, 55], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 253, 170, 25, 214], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectDisplaced(RBX, 887056375, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 253, 173, 25, 171, 247, 103, 223, 52], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 25, 254], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 25, 27], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 253, 201, 25, 208], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 343624300, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 25, 188, 246, 108, 74, 123, 20], OperandSize::Qword)
}

