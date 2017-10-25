use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 231, 21], OperandSize::Dword)
}

#[test]
fn psraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 228, 0], OperandSize::Qword)
}

#[test]
fn psraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 225, 89], OperandSize::Dword)
}

#[test]
fn psraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 224, 2], OperandSize::Qword)
}

#[test]
fn psraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 248], OperandSize::Dword)
}

#[test]
fn psraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 11], OperandSize::Dword)
}

#[test]
fn psraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 217], OperandSize::Qword)
}

#[test]
fn psraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1336430711, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 60, 141, 119, 80, 168, 79], OperandSize::Qword)
}

#[test]
fn psraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 224], OperandSize::Dword)
}

#[test]
fn psraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 28, 72], OperandSize::Dword)
}

#[test]
fn psraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 212], OperandSize::Qword)
}

#[test]
fn psraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 36, 128], OperandSize::Qword)
}

