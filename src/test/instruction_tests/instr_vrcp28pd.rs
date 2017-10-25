use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 157, 202, 211], OperandSize::Dword)
}

#[test]
fn vrcp28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 202, 9], OperandSize::Dword)
}

#[test]
fn vrcp28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 2091095882, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 202, 52, 213, 74, 151, 163, 124], OperandSize::Dword)
}

#[test]
fn vrcp28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 253, 153, 202, 239], OperandSize::Qword)
}

#[test]
fn vrcp28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 412723470, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 206, 202, 28, 205, 14, 169, 153, 24], OperandSize::Qword)
}

#[test]
fn vrcp28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 202, 55], OperandSize::Qword)
}

