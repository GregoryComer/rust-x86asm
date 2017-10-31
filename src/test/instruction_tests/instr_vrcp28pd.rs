use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 202, 194], OperandSize::Dword)
}

#[test]
fn vrcp28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1799939042, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 202, 20, 69, 226, 227, 72, 107], OperandSize::Dword)
}

#[test]
fn vrcp28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 202, 4, 113], OperandSize::Dword)
}

#[test]
fn vrcp28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 253, 156, 202, 227], OperandSize::Qword)
}

#[test]
fn vrcp28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RDX, 82813767, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 202, 146, 71, 163, 239, 4], OperandSize::Qword)
}

#[test]
fn vrcp28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 220, 202, 44, 223], OperandSize::Qword)
}

