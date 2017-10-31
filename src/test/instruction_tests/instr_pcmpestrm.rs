use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 217, 87], OperandSize::Dword)
}

#[test]
fn pcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 11, 76], OperandSize::Dword)
}

#[test]
fn pcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 208, 18], OperandSize::Qword)
}

#[test]
fn pcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 12, 139, 23], OperandSize::Qword)
}

