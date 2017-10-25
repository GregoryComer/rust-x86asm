use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 93, 210], OperandSize::Dword)
}

#[test]
fn vminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 114407292, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 93, 182, 124, 183, 209, 6], OperandSize::Dword)
}

#[test]
fn vminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 93, 251], OperandSize::Qword)
}

#[test]
fn vminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 93, 20, 187], OperandSize::Qword)
}

#[test]
fn vminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 199, 156, 93, 241], OperandSize::Dword)
}

#[test]
fn vminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1052121814, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 223, 140, 93, 60, 77, 214, 26, 182, 62], OperandSize::Dword)
}

#[test]
fn vminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 183, 149, 93, 200], OperandSize::Qword)
}

#[test]
fn vminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 255, 131, 93, 33], OperandSize::Qword)
}

