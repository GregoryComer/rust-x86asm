use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 250, 36], OperandSize::Dword)
}

#[test]
fn blendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 4, 254, 26], OperandSize::Dword)
}

#[test]
fn blendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 207, 91], OperandSize::Qword)
}

#[test]
fn blendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 35, 89], OperandSize::Qword)
}

