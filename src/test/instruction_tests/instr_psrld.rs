use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM0)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 208, 37], OperandSize::Dword)
}

#[test]
fn psrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM4)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 212, 18], OperandSize::Qword)
}

#[test]
fn psrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 208, 33], OperandSize::Dword)
}

#[test]
fn psrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 213, 116], OperandSize::Qword)
}

#[test]
fn psrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 230], OperandSize::Dword)
}

#[test]
fn psrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1200150085, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 172, 242, 69, 214, 136, 71], OperandSize::Dword)
}

#[test]
fn psrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 251], OperandSize::Qword)
}

#[test]
fn psrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1380791992, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 52, 253, 184, 54, 77, 82], OperandSize::Qword)
}

#[test]
fn psrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 217], OperandSize::Dword)
}

#[test]
fn psrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 2034018066, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 140, 198, 18, 167, 60, 121], OperandSize::Dword)
}

#[test]
fn psrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 247], OperandSize::Qword)
}

#[test]
fn psrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 54], OperandSize::Qword)
}

