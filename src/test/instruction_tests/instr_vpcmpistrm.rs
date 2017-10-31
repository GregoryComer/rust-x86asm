use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 253, 79], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 44, 215, 62], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 247, 51], OperandSize::Qword)
}

#[test]
fn vpcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 28, 158, 29], OperandSize::Qword)
}

