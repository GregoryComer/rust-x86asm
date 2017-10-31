use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 153, 204, 232], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EAX, 119403127, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 204, 168, 119, 242, 29, 7], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 204, 60, 217], OperandSize::Dword)
}

#[test]
fn vrsqrt28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 153, 204, 217], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1875648233, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 204, 148, 91, 233, 30, 204, 111], OperandSize::Qword)
}

#[test]
fn vrsqrt28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 220, 204, 36, 121], OperandSize::Qword)
}

