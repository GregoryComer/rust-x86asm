use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 215, 39], OperandSize::Dword)
}

#[test]
fn pcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 4, 209, 59], OperandSize::Dword)
}

#[test]
fn pcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 239, 65], OperandSize::Qword)
}

#[test]
fn pcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 44, 83, 72], OperandSize::Qword)
}

