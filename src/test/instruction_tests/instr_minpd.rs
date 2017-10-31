use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 228], OperandSize::Dword)
}

#[test]
fn minpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 54], OperandSize::Dword)
}

#[test]
fn minpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 221], OperandSize::Qword)
}

#[test]
fn minpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 2039037862, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 156, 183, 166, 63, 137, 121], OperandSize::Qword)
}

