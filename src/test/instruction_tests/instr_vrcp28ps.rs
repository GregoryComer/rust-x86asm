use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 202, 239], OperandSize::Dword)
}

#[test]
fn vrcp28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2115889697, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 202, 20, 141, 33, 234, 29, 126], OperandSize::Dword)
}

#[test]
fn vrcp28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 202, 60, 152], OperandSize::Dword)
}

#[test]
fn vrcp28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 125, 154, 202, 224], OperandSize::Qword)
}

#[test]
fn vrcp28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 202, 44, 119], OperandSize::Qword)
}

#[test]
fn vrcp28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM21)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 219, 202, 41], OperandSize::Qword)
}

