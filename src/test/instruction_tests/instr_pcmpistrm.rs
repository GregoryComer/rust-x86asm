use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpistrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 204, 89], OperandSize::Dword)
}

#[test]
fn pcmpistrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 96583506, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 172, 179, 82, 191, 193, 5, 54], OperandSize::Dword)
}

#[test]
fn pcmpistrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 250, 102], OperandSize::Qword)
}

#[test]
fn pcmpistrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 512237621, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 98, 168, 53, 32, 136, 30, 92], OperandSize::Qword)
}

