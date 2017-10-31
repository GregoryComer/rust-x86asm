use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM2)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 242, 86], OperandSize::Dword)
}

#[test]
fn psllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM2)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 242, 77], OperandSize::Qword)
}

#[test]
fn psllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 242, 109], OperandSize::Dword)
}

#[test]
fn psllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 245, 119], OperandSize::Qword)
}

#[test]
fn psllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 238], OperandSize::Dword)
}

#[test]
fn psllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 36, 214], OperandSize::Dword)
}

#[test]
fn psllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 205], OperandSize::Qword)
}

#[test]
fn psllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 926022164, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 243, 172, 211, 20, 250, 49, 55], OperandSize::Qword)
}

#[test]
fn psllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 219], OperandSize::Dword)
}

#[test]
fn psllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 968232270, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 158, 78, 13, 182, 57], OperandSize::Dword)
}

#[test]
fn psllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 217], OperandSize::Qword)
}

#[test]
fn psllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 358387127, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 243, 36, 93, 183, 141, 92, 21], OperandSize::Qword)
}

