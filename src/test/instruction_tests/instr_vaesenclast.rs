use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 208], OperandSize::Dword)
}

#[test]
fn vaesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 643384314, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 139, 250, 67, 89, 38], OperandSize::Dword)
}

#[test]
fn vaesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 221, 217], OperandSize::Qword)
}

#[test]
fn vaesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 36, 250], OperandSize::Qword)
}

