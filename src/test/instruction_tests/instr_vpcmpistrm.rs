use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 202, 23], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 15, 50], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 245, 84], OperandSize::Qword)
}

#[test]
fn vpcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 32, 30], OperandSize::Qword)
}

