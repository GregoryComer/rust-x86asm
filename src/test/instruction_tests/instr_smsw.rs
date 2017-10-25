use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn smsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 230], OperandSize::Word)
}

#[test]
fn smsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 35], OperandSize::Word)
}

#[test]
fn smsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 228], OperandSize::Dword)
}

#[test]
fn smsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectDisplaced(EAX, 2053386488, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 160, 248, 48, 100, 122], OperandSize::Dword)
}

#[test]
fn smsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 231], OperandSize::Qword)
}

#[test]
fn smsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 34], OperandSize::Qword)
}

#[test]
fn smsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 225], OperandSize::Word)
}

#[test]
fn smsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 109, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 99, 109], OperandSize::Word)
}

#[test]
fn smsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 230], OperandSize::Dword)
}

#[test]
fn smsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 36, 70], OperandSize::Dword)
}

#[test]
fn smsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 230], OperandSize::Qword)
}

#[test]
fn smsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 406867417, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 164, 191, 217, 77, 64, 24], OperandSize::Qword)
}

#[test]
fn smsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(RDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 1, 231], OperandSize::Qword)
}

#[test]
fn smsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 36, 119], OperandSize::Qword)
}

