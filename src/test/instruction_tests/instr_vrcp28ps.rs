use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 202, 233], OperandSize::Dword)
}

#[test]
fn vrcp28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 202, 36, 201], OperandSize::Dword)
}

#[test]
fn vrcp28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1322586480, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 202, 12, 141, 112, 17, 213, 78], OperandSize::Dword)
}

#[test]
fn vrcp28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 153, 202, 219], OperandSize::Qword)
}

#[test]
fn vrcp28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectDisplaced(RSI, 1881101257, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 202, 134, 201, 83, 31, 112], OperandSize::Qword)
}

#[test]
fn vrcp28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1688623049, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 221, 202, 140, 75, 201, 87, 166, 100], OperandSize::Qword)
}

