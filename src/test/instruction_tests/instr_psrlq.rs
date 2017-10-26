use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM0)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 208, 106], OperandSize::Dword)
}

#[test]
fn psrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM7)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 215, 4], OperandSize::Qword)
}

#[test]
fn psrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 215, 80], OperandSize::Dword)
}

#[test]
fn psrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 210, 126], OperandSize::Qword)
}

#[test]
fn psrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 247], OperandSize::Dword)
}

#[test]
fn psrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 20, 66], OperandSize::Dword)
}

#[test]
fn psrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 218], OperandSize::Qword)
}

#[test]
fn psrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 803628928, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 28, 253, 128, 103, 230, 47], OperandSize::Qword)
}

#[test]
fn psrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 215], OperandSize::Dword)
}

#[test]
fn psrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1898107733, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 4, 197, 85, 211, 34, 113], OperandSize::Dword)
}

#[test]
fn psrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 255], OperandSize::Qword)
}

#[test]
fn psrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 3], OperandSize::Qword)
}

