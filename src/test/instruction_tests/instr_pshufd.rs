use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 233, 110], OperandSize::Dword)
}

#[test]
fn pshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDI, 133588337, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 143, 113, 101, 246, 7, 1], OperandSize::Dword)
}

#[test]
fn pshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 226, 75], OperandSize::Qword)
}

#[test]
fn pshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1828895947, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 12, 85, 203, 188, 2, 109, 109], OperandSize::Qword)
}

