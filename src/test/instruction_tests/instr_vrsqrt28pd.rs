use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 204, 200], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1786891593, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 204, 140, 249, 73, 205, 129, 106], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 906755054, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 204, 148, 151, 238, 251, 11, 54], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 253, 159, 204, 235], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 2035612232, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 253, 202, 204, 20, 189, 72, 250, 84, 121], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 204, 34], OperandSize::Qword)
}

