use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 241, 113], OperandSize::Dword)
}

#[test]
fn pcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 4, 214, 37], OperandSize::Dword)
}

#[test]
fn pcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 238, 127], OperandSize::Qword)
}

#[test]
fn pcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1078861347, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 164, 147, 35, 30, 78, 64, 15], OperandSize::Qword)
}

