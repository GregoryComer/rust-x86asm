use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 156, 202, 213], OperandSize::Dword)
}

#[test]
fn vrcp28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 202, 20, 144], OperandSize::Dword)
}

#[test]
fn vrcp28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(ESI, 231088131, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 202, 134, 3, 32, 198, 13], OperandSize::Dword)
}

#[test]
fn vrcp28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 202, 192], OperandSize::Qword)
}

#[test]
fn vrcp28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectDisplaced(RDI, 40622001, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 202, 183, 177, 215, 107, 2], OperandSize::Qword)
}

#[test]
fn vrcp28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 221, 202, 25], OperandSize::Qword)
}

