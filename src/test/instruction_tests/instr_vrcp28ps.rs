use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 202, 194], OperandSize::Dword)
}

#[test]
fn vrcp28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 381258274, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 202, 148, 126, 34, 138, 185, 22], OperandSize::Dword)
}

#[test]
fn vrcp28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDX, 408102081, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 202, 186, 193, 36, 83, 24], OperandSize::Dword)
}

#[test]
fn vrcp28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 125, 159, 202, 203], OperandSize::Qword)
}

#[test]
fn vrcp28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectDisplaced(RAX, 1578215570, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 202, 168, 146, 168, 17, 94], OperandSize::Qword)
}

#[test]
fn vrcp28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 220, 202, 52, 210], OperandSize::Qword)
}

