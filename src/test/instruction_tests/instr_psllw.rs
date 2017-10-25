use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM5)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 245, 89], OperandSize::Dword)
}

#[test]
fn psllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 244, 10], OperandSize::Qword)
}

#[test]
fn psllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 245, 117], OperandSize::Dword)
}

#[test]
fn psllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 245, 110], OperandSize::Qword)
}

#[test]
fn psllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 255], OperandSize::Dword)
}

#[test]
fn psllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 2123249701, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 188, 211, 37, 56, 142, 126], OperandSize::Dword)
}

#[test]
fn psllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 199], OperandSize::Qword)
}

#[test]
fn psllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RBX, 705440302, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 147, 46, 42, 12, 42], OperandSize::Qword)
}

#[test]
fn psllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 197], OperandSize::Dword)
}

#[test]
fn psllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1037855025, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 140, 203, 49, 105, 220, 61], OperandSize::Dword)
}

#[test]
fn psllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 198], OperandSize::Qword)
}

#[test]
fn psllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 25], OperandSize::Qword)
}

