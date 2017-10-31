use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 220, 0], OperandSize::Dword)
}

#[test]
fn pshufd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 20, 198, 16], OperandSize::Dword)
}

#[test]
fn pshufd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 237, 23], OperandSize::Qword)
}

#[test]
fn pshufd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 764069355, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 112, 164, 144, 235, 197, 138, 45, 102], OperandSize::Qword)
}

