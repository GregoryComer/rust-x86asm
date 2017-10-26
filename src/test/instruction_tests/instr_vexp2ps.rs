use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 200, 250], OperandSize::Dword)
}

#[test]
fn vexp2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1934308283, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 200, 44, 181, 187, 51, 75, 115], OperandSize::Dword)
}

#[test]
fn vexp2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 200, 49], OperandSize::Dword)
}

#[test]
fn vexp2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 125, 155, 200, 245], OperandSize::Qword)
}

#[test]
fn vexp2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 990005653, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 200, 188, 154, 149, 73, 2, 59], OperandSize::Qword)
}

#[test]
fn vexp2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectDisplaced(RDX, 1965967115, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 217, 200, 178, 11, 71, 46, 117], OperandSize::Qword)
}

