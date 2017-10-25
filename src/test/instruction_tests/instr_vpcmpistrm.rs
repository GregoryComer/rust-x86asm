use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 234, 5], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 14, 94], OperandSize::Dword)
}

#[test]
fn vpcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 219, 115], OperandSize::Qword)
}

#[test]
fn vpcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRM, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1245425524, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 98, 12, 149, 116, 175, 59, 74, 66], OperandSize::Qword)
}

