use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 207, 53], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 28, 112, 47], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 229, 24], OperandSize::Qword)
}

#[test]
fn vpcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 60, 87, 47], OperandSize::Qword)
}

