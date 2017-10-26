use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 215, 57], OperandSize::Dword)
}

#[test]
fn psrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 210, 40], OperandSize::Qword)
}

#[test]
fn psrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 210, 123], OperandSize::Dword)
}

#[test]
fn psrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 213, 113], OperandSize::Qword)
}

#[test]
fn psrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 196], OperandSize::Dword)
}

#[test]
fn psrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 39], OperandSize::Dword)
}

#[test]
fn psrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 252], OperandSize::Qword)
}

#[test]
fn psrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 2114290249, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 20, 93, 73, 130, 5, 126], OperandSize::Qword)
}

#[test]
fn psrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 246], OperandSize::Dword)
}

#[test]
fn psrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 41], OperandSize::Dword)
}

#[test]
fn psrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 198], OperandSize::Qword)
}

#[test]
fn psrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 2], OperandSize::Qword)
}

