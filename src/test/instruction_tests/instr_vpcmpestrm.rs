use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 233, 52], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 630042273, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 188, 115, 161, 174, 141, 37, 124], OperandSize::Dword)
}

#[test]
fn vpcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 212, 26], OperandSize::Qword)
}

#[test]
fn vpcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 46, 120], OperandSize::Qword)
}

