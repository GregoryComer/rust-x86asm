use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 153, 202, 198], OperandSize::Dword)
}

#[test]
fn vrcp28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(ECX, 900219752, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 202, 185, 104, 67, 168, 53], OperandSize::Dword)
}

#[test]
fn vrcp28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 515943454, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 202, 148, 208, 30, 172, 192, 30], OperandSize::Dword)
}

#[test]
fn vrcp28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 253, 156, 202, 223], OperandSize::Qword)
}

#[test]
fn vrcp28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 205, 202, 4, 65], OperandSize::Qword)
}

#[test]
fn vrcp28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RBX, 1082739589, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 219, 202, 155, 133, 75, 137, 64], OperandSize::Qword)
}

