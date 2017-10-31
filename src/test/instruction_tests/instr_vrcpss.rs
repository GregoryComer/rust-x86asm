use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 83, 209], OperandSize::Dword)
}

#[test]
fn vrcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1879310759, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 83, 44, 189, 167, 1, 4, 112], OperandSize::Dword)
}

#[test]
fn vrcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 83, 204], OperandSize::Qword)
}

#[test]
fn vrcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 83, 28, 144], OperandSize::Qword)
}

