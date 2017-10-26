use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 159, 202, 214], OperandSize::Dword)
}

#[test]
fn vrcp28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 202, 4, 138], OperandSize::Dword)
}

#[test]
fn vrcp28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 202, 12, 191], OperandSize::Dword)
}

#[test]
fn vrcp28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 158, 202, 241], OperandSize::Qword)
}

#[test]
fn vrcp28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(RCX, 1697046383, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 202, 185, 111, 223, 38, 101], OperandSize::Qword)
}

#[test]
fn vrcp28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 1397565989, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 221, 202, 172, 251, 37, 42, 77, 83], OperandSize::Qword)
}

