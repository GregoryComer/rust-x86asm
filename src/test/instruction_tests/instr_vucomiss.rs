use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 238], OperandSize::Dword)
}

#[test]
fn vucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1299623701, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 52, 189, 21, 175, 118, 77], OperandSize::Dword)
}

#[test]
fn vucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 217], OperandSize::Qword)
}

#[test]
fn vucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 29201873, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 168, 209, 149, 189, 1], OperandSize::Qword)
}

#[test]
fn vucomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 46, 243], OperandSize::Dword)
}

#[test]
fn vucomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1048172686, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 4, 149, 142, 216, 121, 62], OperandSize::Dword)
}

#[test]
fn vucomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 97, 124, 24, 46, 214], OperandSize::Qword)
}

#[test]
fn vucomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 332218665, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 46, 44, 253, 41, 65, 205, 19], OperandSize::Qword)
}

