use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 221, 194], OperandSize::Dword)
}

#[test]
fn vaesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 806789905, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 221, 4, 197, 17, 163, 22, 48], OperandSize::Dword)
}

#[test]
fn vaesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 221, 254], OperandSize::Qword)
}

#[test]
fn vaesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 361301546, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 221, 132, 218, 42, 6, 137, 21], OperandSize::Qword)
}

