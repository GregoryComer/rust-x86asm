use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 251, 109], OperandSize::Dword)
}

#[test]
fn pcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 299469715, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 156, 255, 147, 139, 217, 17, 40], OperandSize::Dword)
}

#[test]
fn pcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 213, 83], OperandSize::Qword)
}

#[test]
fn pcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 354655536, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 36, 197, 48, 157, 35, 21, 11], OperandSize::Qword)
}

