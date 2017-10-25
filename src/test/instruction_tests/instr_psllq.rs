use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 243, 116], OperandSize::Dword)
}

#[test]
fn psllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM5)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 245, 26], OperandSize::Qword)
}

#[test]
fn psllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 242, 64], OperandSize::Dword)
}

#[test]
fn psllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 247, 126], OperandSize::Qword)
}

#[test]
fn psllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 217], OperandSize::Dword)
}

#[test]
fn psllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 18], OperandSize::Dword)
}

#[test]
fn psllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 220], OperandSize::Qword)
}

#[test]
fn psllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(RDI, 1604939470, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 159, 206, 110, 169, 95], OperandSize::Qword)
}

#[test]
fn psllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 196], OperandSize::Dword)
}

#[test]
fn psllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1981659297, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 172, 118, 161, 184, 29, 118], OperandSize::Dword)
}

#[test]
fn psllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 192], OperandSize::Qword)
}

#[test]
fn psllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 60, 201], OperandSize::Qword)
}

