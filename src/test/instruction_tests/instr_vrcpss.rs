use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 83, 232], OperandSize::Dword)
}

#[test]
fn vrcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 83, 4, 90], OperandSize::Dword)
}

#[test]
fn vrcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 83, 223], OperandSize::Qword)
}

#[test]
fn vrcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 345343125, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 83, 156, 153, 149, 132, 149, 20], OperandSize::Qword)
}

