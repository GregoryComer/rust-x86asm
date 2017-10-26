use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 83, 228], OperandSize::Dword)
}

#[test]
fn vrcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 83, 25], OperandSize::Dword)
}

#[test]
fn vrcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 83, 244], OperandSize::Qword)
}

#[test]
fn vrcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1817368434, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 83, 180, 123, 114, 215, 82, 108], OperandSize::Qword)
}

