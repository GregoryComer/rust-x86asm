use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 197], OperandSize::Dword)
}

#[test]
fn psubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 52, 179], OperandSize::Dword)
}

#[test]
fn psubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 194], OperandSize::Qword)
}

#[test]
fn psubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 15], OperandSize::Qword)
}

#[test]
fn psubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 244], OperandSize::Dword)
}

#[test]
fn psubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 36, 215], OperandSize::Dword)
}

#[test]
fn psubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 194], OperandSize::Qword)
}

#[test]
fn psubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 24], OperandSize::Qword)
}

